"""Tests for build.py — run with: python3 -m unittest"""

import json
import re
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import build


SAMPLE_PR = {
    "number": 100,
    "rank": 1,
    "url": "https://github.com/NethermindEth/nethermind/pull/100",
    "tag": "Core",
    "title": "Sample title",
    "diff": {"add": 10, "del": 2},
    "merged": "2026-01-02",
    "description": 'Sample `Code` description with an & ampersand and a "quote".',
}


def make_pr(**overrides):
    pr = json.loads(json.dumps(SAMPLE_PR))  # cheap deep copy
    pr.update(overrides)
    return pr


TEMPLATE = '<div class="card-grid">\n\n' + build.MARKER_LINE + "\n      </div>\n"
INDEX_TEMPLATE = TEMPLATE + build.LINK_MARKER_LINE


class RenderInlineTests(unittest.TestCase):
    def test_escapes_html_special_characters(self):
        self.assertEqual(build.render_inline("A & B < C > D"), "A &amp; B &lt; C &gt; D")

    def test_preserves_apostrophes_and_quotes(self):
        self.assertEqual(build.render_inline("it's a \"test\""), "it's a \"test\"")

    def test_converts_backtick_span_to_code_tag(self):
        self.assertEqual(build.render_inline("call `foo()` now"), "call <code>foo()</code> now")

    def test_escapes_content_inside_code_span(self):
        self.assertEqual(build.render_inline("`A & B`"), "<code>A &amp; B</code>")

    def test_multiple_code_spans(self):
        self.assertEqual(build.render_inline("`a` and `b`"), "<code>a</code> and <code>b</code>")


class FormatMergedDateTests(unittest.TestCase):
    def test_formats_two_digit_day(self):
        self.assertEqual(build.format_merged_date("2026-09-25"), "Sep 25, 2026")

    def test_formats_single_digit_day_without_leading_zero(self):
        self.assertEqual(build.format_merged_date("2026-09-07"), "Sep 7, 2026")

    def test_invalid_date_raises_build_error(self):
        with self.assertRaises(build.BuildError):
            build.format_merged_date("not-a-date")

    def test_out_of_range_month_raises_build_error(self):
        with self.assertRaises(build.BuildError):
            build.format_merged_date("2026-13-01")


class SortPrsTests(unittest.TestCase):
    def test_sorts_newest_date_first(self):
        older = make_pr(number=1, merged="2026-01-01")
        newer = make_pr(number=2, merged="2026-02-01")
        ordered = build.sort_prs([older, newer])
        self.assertEqual([pr["number"] for pr in ordered], [2, 1])

    def test_ascending_number_within_same_date(self):
        high = make_pr(number=200, merged="2026-01-01")
        low = make_pr(number=100, merged="2026-01-01")
        ordered = build.sort_prs([high, low])
        self.assertEqual([pr["number"] for pr in ordered], [100, 200])

    def test_mixed_dates_and_numbers(self):
        prs = [
            make_pr(number=13833, merged="2026-09-25"),
            make_pr(number=13506, merged="2026-09-25"),
            make_pr(number=13485, merged="2026-09-23"),
            make_pr(number=13366, merged="2026-09-23"),
        ]
        ordered = build.sort_prs(prs)
        self.assertEqual(
            [pr["number"] for pr in ordered],
            [13506, 13833, 13366, 13485],
        )


class RenderCardTests(unittest.TestCase):
    def test_heading_level_is_configurable(self):
        rendered = build.render_card(make_pr(), heading_level=2)
        self.assertIn("<h2>Sample title</h2>", rendered)
        self.assertNotIn("<h3>", rendered)

    def test_renders_expected_fields(self):
        rendered = build.render_card(make_pr())
        self.assertIn('href="https://github.com/NethermindEth/nethermind/pull/100"', rendered)
        self.assertIn("#100</a>", rendered)
        self.assertIn('<span class="tag">Core</span>', rendered)
        self.assertIn("<h3>Sample title</h3>", rendered)
        self.assertIn('<span class="add">+10</span>', rendered)
        self.assertIn('<span class="del">-2</span>', rendered)
        self.assertIn("Merged Jan 2, 2026", rendered)
        self.assertIn("<code>Code</code>", rendered)
        self.assertIn("&amp; ampersand", rendered)


def ranked_prs(count):
    """`count` PRs whose rank is the reverse of their merge-date order."""
    return [
        make_pr(number=1000 + i, rank=count - i, merged=f"2026-01-{i + 1:02d}")
        for i in range(count)
    ]


class SelectFeaturedTests(unittest.TestCase):
    def test_orders_by_rank_ascending(self):
        prs = [make_pr(number=1, rank=3), make_pr(number=2, rank=1), make_pr(number=3, rank=2)]
        self.assertEqual([pr["rank"] for pr in build.select_featured(prs)], [1, 2, 3])

    def test_excludes_entries_ranked_beyond_featured_count(self):
        featured = build.select_featured(ranked_prs(build.FEATURED_COUNT + 2))
        self.assertEqual(
            [pr["rank"] for pr in featured], list(range(1, build.FEATURED_COUNT + 1))
        )

    def test_exactly_featured_count_entries_are_all_kept(self):
        self.assertEqual(
            len(build.select_featured(ranked_prs(build.FEATURED_COUNT))), build.FEATURED_COUNT
        )

    def test_fewer_than_featured_count_entries_are_all_kept(self):
        self.assertEqual(len(build.select_featured(ranked_prs(4))), 4)

    def test_empty_input_gives_empty_list(self):
        self.assertEqual(build.select_featured([]), [])


class AllPrsLinkTests(unittest.TestCase):
    def test_link_points_to_prs_page_and_shows_total(self):
        rendered = build.render_all_prs_link(17)
        self.assertIn('href="prs.html"', rendered)
        self.assertIn("See all 17 merged PRs", rendered)

    def test_link_uses_singular_for_one_pr(self):
        self.assertIn("See all 1 merged PR", build.render_all_prs_link(1))
        self.assertNotIn("PRs", build.render_all_prs_link(1))


class BuildIndexPageTests(unittest.TestCase):
    def setUp(self):
        self.prs = ranked_prs(build.FEATURED_COUNT + 2)
        self.page = build.build_index_page(INDEX_TEMPLATE, self.prs)

    def test_shows_only_featured_cards(self):
        self.assertEqual(self.page.count('<article class="card pr-card">'), build.FEATURED_COUNT)

    def test_orders_cards_by_rank(self):
        numbers = [int(n) for n in re.findall(r"#(\d+)</a>", self.page)]
        self.assertEqual(numbers, [pr["number"] for pr in build.select_featured(self.prs)])

    def test_places_link_after_the_cards(self):
        self.assertLess(
            self.page.rindex("</article>"), self.page.index('<p class="section-cta">')
        )
        self.assertIn("See all 17 merged PRs", self.page)

    def test_removes_link_marker(self):
        self.assertNotIn("<!-- ALL_PRS_LINK -->", self.page)

    def test_cards_use_h3_headings(self):
        self.assertEqual(self.page.count("<h3>"), build.FEATURED_COUNT)
        self.assertNotIn("<h2>", self.page)

    def test_missing_link_marker_raises_build_error(self):
        with self.assertRaises(build.BuildError):
            build.build_index_page(TEMPLATE, [make_pr()])

    def test_duplicate_link_marker_raises_build_error(self):
        with self.assertRaises(build.BuildError):
            build.build_index_page(INDEX_TEMPLATE + build.LINK_MARKER_LINE, [make_pr()])


class BuildPrsPageTests(unittest.TestCase):
    def test_lists_every_entry_newest_first(self):
        prs = ranked_prs(build.FEATURED_COUNT + 2)
        page = build.build_prs_page(TEMPLATE, prs)
        numbers = [int(n) for n in re.findall(r"#(\d+)</a>", page)]
        self.assertEqual(numbers, [pr["number"] for pr in build.sort_prs(prs)])
        self.assertEqual(len(numbers), len(prs))

    def test_cards_use_h2_headings(self):
        page = build.build_prs_page(TEMPLATE, [make_pr()])
        self.assertIn("<h2>Sample title</h2>", page)
        self.assertNotIn("<h3>", page)

    def test_missing_marker_raises_build_error(self):
        with self.assertRaises(build.BuildError):
            build.build_prs_page("<main></main>", [make_pr()])

    def test_stray_link_marker_raises_build_error(self):
        with self.assertRaisesRegex(build.BuildError, "ALL_PRS_LINK"):
            build.build_prs_page(INDEX_TEMPLATE, [make_pr()])


class BuildPageTests(unittest.TestCase):
    def test_replaces_marker_with_rendered_cards(self):
        page = build.build_page(TEMPLATE, [make_pr()])
        self.assertNotIn("<!-- PR_CARDS -->", page)
        self.assertIn('<article class="card pr-card">', page)

    def test_missing_marker_raises_build_error(self):
        with self.assertRaises(build.BuildError):
            build.build_page('<div class="card-grid"></div>', [make_pr()])

    def test_duplicate_marker_raises_build_error(self):
        with self.assertRaises(build.BuildError):
            build.build_page(TEMPLATE + TEMPLATE, [make_pr()])


class LoadPrsTests(unittest.TestCase):
    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmpdir.cleanup)
        self.path = Path(self.tmpdir.name) / "prs.json"

    def write(self, content):
        self.path.write_text(content, encoding="utf-8")

    def test_loads_valid_entries(self):
        self.write(json.dumps([SAMPLE_PR]))
        prs = build.load_prs(self.path)
        self.assertEqual(len(prs), 1)
        self.assertEqual(prs[0]["number"], 100)

    def test_malformed_json_raises_build_error(self):
        self.write("{not valid json")
        with self.assertRaises(build.BuildError):
            build.load_prs(self.path)

    def test_non_list_root_raises_build_error(self):
        self.write(json.dumps({"not": "a list"}))
        with self.assertRaises(build.BuildError):
            build.load_prs(self.path)

    def test_missing_field_raises_build_error(self):
        broken = make_pr()
        del broken["title"]
        self.write(json.dumps([broken]))
        with self.assertRaises(build.BuildError):
            build.load_prs(self.path)

    def test_malformed_diff_raises_build_error(self):
        self.write(json.dumps([make_pr(diff={"add": 1})]))
        with self.assertRaises(build.BuildError):
            build.load_prs(self.path)

    def test_negative_diff_value_raises_build_error(self):
        self.write(json.dumps([make_pr(diff={"add": -1, "del": 2})]))
        with self.assertRaises(build.BuildError):
            build.load_prs(self.path)

    def test_non_integer_diff_value_raises_build_error(self):
        self.write(json.dumps([make_pr(diff={"add": "10", "del": 2})]))
        with self.assertRaises(build.BuildError):
            build.load_prs(self.path)

    def test_non_integer_number_raises_build_error(self):
        self.write(json.dumps([make_pr(number="100")]))
        with self.assertRaises(build.BuildError):
            build.load_prs(self.path)

    def test_empty_string_field_raises_build_error(self):
        self.write(json.dumps([make_pr(title="")]))
        with self.assertRaises(build.BuildError):
            build.load_prs(self.path)

    def test_missing_rank_raises_build_error(self):
        pr = make_pr()
        del pr["rank"]
        self.write(json.dumps([pr]))
        with self.assertRaisesRegex(build.BuildError, "rank"):
            build.load_prs(self.path)

    def test_non_integer_rank_raises_build_error(self):
        for bad_rank in ("1", 1.5, None, True):
            with self.subTest(rank=bad_rank):
                self.write(json.dumps([make_pr(rank=bad_rank)]))
                with self.assertRaisesRegex(build.BuildError, "rank"):
                    build.load_prs(self.path)

    def test_zero_or_negative_rank_raises_build_error(self):
        for bad_rank in (0, -3):
            with self.subTest(rank=bad_rank):
                self.write(json.dumps([make_pr(rank=bad_rank)]))
                with self.assertRaisesRegex(build.BuildError, "rank"):
                    build.load_prs(self.path)

    def test_duplicate_rank_raises_build_error_naming_both_entries(self):
        first = make_pr(number=1, rank=2)
        second = make_pr(number=2, rank=2)
        self.write(json.dumps([first, second]))
        with self.assertRaisesRegex(build.BuildError, r"duplicate rank 2.*#1.*#2"):
            build.load_prs(self.path)

    def test_rank_gap_raises_build_error_naming_missing_rank(self):
        self.write(json.dumps([make_pr(number=1, rank=1), make_pr(number=2, rank=3)]))
        with self.assertRaisesRegex(build.BuildError, r"missing.*\b2\b"):
            build.load_prs(self.path)

    def test_ranks_not_starting_at_one_raise_build_error(self):
        self.write(json.dumps([make_pr(number=1, rank=2), make_pr(number=2, rank=3)]))
        with self.assertRaisesRegex(build.BuildError, r"missing.*\b1\b"):
            build.load_prs(self.path)

    def test_rank_beyond_entry_count_raises_build_error_naming_it(self):
        self.write(json.dumps([make_pr(number=1, rank=1), make_pr(number=2, rank=5)]))
        with self.assertRaisesRegex(build.BuildError, r"unexpected.*\b5\b"):
            build.load_prs(self.path)

    def test_missing_file_raises_build_error(self):
        with self.assertRaises(build.BuildError):
            build.load_prs(Path(self.tmpdir.name) / "does-not-exist.json")


class BuildIntegrationTests(unittest.TestCase):
    """Exercises build() end-to-end against a temporary project layout."""

    def setUp(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmpdir.cleanup)
        root = Path(self.tmpdir.name)

        (root / "src").mkdir()
        (root / "data").mkdir()
        (root / "assets" / "css").mkdir(parents=True)
        (root / "assets" / "css" / "style.css").write_text("body {}", encoding="utf-8")
        (root / ".nojekyll").write_text("", encoding="utf-8")

        (root / "src" / "index.html").write_text(INDEX_TEMPLATE, encoding="utf-8")
        (root / "src" / "prs.html").write_text(TEMPLATE, encoding="utf-8")
        (root / "data" / "prs.json").write_text(json.dumps([SAMPLE_PR]), encoding="utf-8")

        self._originals = {
            "TEMPLATE_PATH": build.TEMPLATE_PATH,
            "PRS_TEMPLATE_PATH": build.PRS_TEMPLATE_PATH,
            "DATA_PATH": build.DATA_PATH,
            "DIST_DIR": build.DIST_DIR,
            "ASSETS_DIR": build.ASSETS_DIR,
            "NOJEKYLL_PATH": build.NOJEKYLL_PATH,
        }
        build.TEMPLATE_PATH = root / "src" / "index.html"
        build.PRS_TEMPLATE_PATH = root / "src" / "prs.html"
        build.DATA_PATH = root / "data" / "prs.json"
        build.DIST_DIR = root / "dist"
        build.ASSETS_DIR = root / "assets"
        build.NOJEKYLL_PATH = root / ".nojekyll"
        self.addCleanup(self._restore_paths)

    def _restore_paths(self):
        for name, value in self._originals.items():
            setattr(build, name, value)

    def test_writes_rendered_page_and_copies_static_files(self):
        build.build()
        output = build.DIST_DIR / "index.html"
        self.assertTrue(output.is_file())
        rendered = output.read_text(encoding="utf-8")
        self.assertIn('<article class="card pr-card">', rendered)
        self.assertIn("#100</a>", rendered)
        self.assertTrue((build.DIST_DIR / "assets" / "css" / "style.css").is_file())
        self.assertTrue((build.DIST_DIR / ".nojekyll").is_file())

    def test_writes_full_pr_listing_page(self):
        build.build()
        rendered = (build.DIST_DIR / "prs.html").read_text(encoding="utf-8")
        self.assertIn("#100</a>", rendered)
        self.assertNotIn("<!-- PR_CARDS -->", rendered)

    def test_index_page_shows_featured_subset_and_link(self):
        prs = ranked_prs(build.FEATURED_COUNT + 2)
        build.DATA_PATH.write_text(json.dumps(prs), encoding="utf-8")
        build.build()
        index = (build.DIST_DIR / "index.html").read_text(encoding="utf-8")
        prs_page = (build.DIST_DIR / "prs.html").read_text(encoding="utf-8")
        self.assertEqual(index.count('<article class="card pr-card">'), build.FEATURED_COUNT)
        self.assertEqual(prs_page.count('<article class="card pr-card">'), len(prs))
        self.assertIn('href="prs.html"', index)
        self.assertNotIn("<!-- ALL_PRS_LINK -->", index)

    def test_raises_on_missing_prs_template(self):
        build.PRS_TEMPLATE_PATH.unlink()
        with self.assertRaises(build.BuildError):
            build.build()

    def test_raises_on_missing_data_file(self):
        build.DATA_PATH.unlink()
        with self.assertRaises(build.BuildError):
            build.build()


if __name__ == "__main__":
    unittest.main()
