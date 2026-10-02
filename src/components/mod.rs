//! Small view components: one per section or card.

mod bug_bounty;
mod ensimag;
mod footer;
mod header;
mod hero;
mod inline_text;
mod layout;
mod nethermind;
mod not_found;
mod postage;
mod pr_card;
mod pr_error;
mod section;
mod theme_toggle;

pub use bug_bounty::BugBountySection;
pub use ensimag::EnsimagSection;
pub use footer::Footer;
pub use header::Header;
pub use hero::Hero;
pub use inline_text::InlineText;
pub use layout::PageShell;
pub use nethermind::NethermindSection;
pub use not_found::NotFound;
pub use postage::PostageSection;
pub use pr_card::PrCard;
pub use pr_error::PrsError;
pub use section::Section;
pub use theme_toggle::ThemeToggle;
