mod run;
pub use run::run;

mod once;
pub use once::once;

mod list;
pub use list::list;

mod report;
pub use report::report;

mod generate_completions;
pub use generate_completions::generate_completions;

mod mutators;
pub use mutators::add;
pub use mutators::remove;

#[cfg(test)]
mod tests {
    const TEST_URL: &str = "https://crates.io/";
    use crate::configuration::{MonitorConfiguration, MonitorRule};
    fn mock_configuration() -> MonitorConfiguration {
        return MonitorConfiguration {
            rules: vec![MonitorRule {
                url: String::from(TEST_URL),
            }],
        };
    }

    // TODO: these tests need to assert something.
    // TODO: these tests need to mock the is_up core.

    #[test]
    fn test_once() {
        super::once(TEST_URL);
    }
    #[test]
    fn test_run() {
        super::run(mock_configuration())
    }

    #[test]
    fn test_list() {
        super::list(mock_configuration());
    }
    #[test]
    fn test_report() {
        super::report(mock_configuration());
    }

    #[test]
    fn test_generate_completions() {
        super::generate_completions("bash");
    }

    // TODO: test add and remove from JSON file.
}
