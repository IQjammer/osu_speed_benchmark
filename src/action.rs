#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    StartBenchmark,
    RegisterClick(char),
    Reset,
    Tick,
    None,
}
