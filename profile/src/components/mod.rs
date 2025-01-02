mod blog;
pub use blog::{BlogSection, BookSection, BlogLink};

mod card;
pub use card::{CardSection, Education, KnownColleges};

mod code;
pub use code::CodeBlock;

mod footer;
pub use footer::{Footer, FooterItem};

mod gallery;
pub use gallery::{Gallery, Media};

mod list;
pub use list::{DescriptionList, Category, Skill};

mod navbar;
pub use navbar::{NavBar, NavItem};

// mod navbar;
// pub use navbar::NavBar;

mod profile;
pub use profile::{UserProfile, Contact, ContactType, Biography};

mod section;
pub use section::Section;

// mod section;
// pub use section::Section;

mod testimonial;
pub use testimonial::{Testimonials, Testimonial};

mod timeline;
pub use timeline::{Timeline, Feed, KnownLogo, CardBlog};