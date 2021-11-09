pub mod logger {
    use std::{
        fmt::Display,
        panic,
        sync::atomic::{AtomicU16, Ordering},
    };

    use difference::{Changeset, Difference};
    use yansi::{Color, Paint, Style};

    static ACTION_COUNT: AtomicU16 = AtomicU16::new(0);

    fn with_prefix_and_style<S1, S2>(text: S1, prefix: S2, style: Style) -> String
    where
        S1: Display,
        S2: Display,
    {
        text.to_string()
            // .trim_end()
            .split('\n')
            .map(|line| {
                format!(
                    "{}{}",
                    prefix.to_string(),
                    Paint::new(line).with_style(style)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
            .to_owned()
    }

    fn with_prefix<S1, S2>(text: S1, prefix: S2) -> String
    where
        S1: Display,
        S2: Display,
    {
        with_prefix_and_style(text, prefix, Style::default())
    }

    fn get_line_prefix() -> String {
        format!(
            "{}",
            "  │  ".repeat(ACTION_COUNT.load(Ordering::SeqCst).into())
        )
    }

    pub fn log<S>(message: S)
    where
        S: Display,
    {
        let line_prefix = get_line_prefix();
        // println!("{:?}", with_prefix(&message, &line_prefix));
        println!("{}", with_prefix(&message, &line_prefix));
    }

    pub fn start_action<S>(title: S)
    where
        S: Display,
    {
        log(title);
        log("  ╷");
        ACTION_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    pub fn end_action<S>(message: S)
    where
        S: Display,
    {
        if ACTION_COUNT.load(Ordering::SeqCst) == 0 {
            panic!("Attempted to end an action that was not started.");
        }

        log("");
        ACTION_COUNT.fetch_sub(1, Ordering::SeqCst);
        log(&format!("  ╰─ {}", message));
    }

    pub fn end_action_with_results<S1, S2>(message: S1, results: S2)
    where
        S1: Display,
        S2: Display,
    {
        end_action(message);
        log(&with_prefix_and_style(
            results,
            "       ",
            Style::default().dimmed(),
        ));
    }

    pub fn log_changeset(changeset: Changeset) {
        log(&changeset
            .diffs
            .iter()
            .map(|diff| match diff {
                Difference::Same(same) => {
                    with_prefix_and_style(same, "  ", Style::default().dimmed())
                }
                Difference::Add(add) => with_prefix_and_style(
                    add,
                    &format!("{} ", Paint::green("+")),
                    Style::new(Color::Green),
                ),
                Difference::Rem(rem) => with_prefix_and_style(
                    rem,
                    &format!("{} ", Paint::red("-")),
                    Style::new(Color::Red),
                ),
            })
            .collect::<Vec<String>>()
            .join(&changeset.split));
    }
}
