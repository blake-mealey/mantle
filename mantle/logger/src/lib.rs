use std::{
    fmt::Display,
    panic,
    sync::atomic::{AtomicU16, Ordering},
};

use difference::{Changeset, Difference};
use yansi::{Color, Paint, Style};

const SPACING: &str = "  ";

static ACTION_COUNT: AtomicU16 = AtomicU16::new(0);

fn with_prefix_and_style<S1, S2>(text: S1, prefix: S2, style: Style) -> String
where
    S1: Display,
    S2: Display,
{
    text.to_string()
        .split('\n')
        .map(|line| format!("{}{}", prefix, Paint::new(line).with_style(style)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn with_prefix<S1, S2>(text: S1, prefix: S2) -> String
where
    S1: Display,
    S2: Display,
{
    with_prefix_and_style(text, prefix, Style::default())
}

fn get_line_prefix() -> String {
    format!("{SPACING}│{SPACING}").repeat(ACTION_COUNT.load(Ordering::SeqCst).into())
}

pub fn log<S>(message: S)
where
    S: Display,
{
    let line_prefix = get_line_prefix();
    println!("{}", with_prefix(&message, line_prefix));
}

pub fn start_action<S>(title: S)
where
    S: Display,
{
    log(title);
    log("  ╷");
    ACTION_COUNT.fetch_add(1, Ordering::SeqCst);
}

fn end_action_internal<S>(message: Option<S>, results: Option<Changeset>)
where
    S: Display,
{
    if ACTION_COUNT.load(Ordering::SeqCst) == 0 {
        panic!("Attempted to end an action that was not started.");
    }

    log("");
    ACTION_COUNT.fetch_sub(1, Ordering::SeqCst);

    if let Some(message) = message {
        log(format!("{SPACING}╰─ {message}"));
    } else {
        log(format!("{SPACING}╰──○"));
    }

    if let Some(results) = results {
        log_changeset_with_prefix(results, format!("{SPACING}{SPACING} "));
    }

    log("");
}

pub fn end_action<S>(message: S)
where
    S: Display,
{
    end_action_internal(Some(message), None);
}

pub fn end_action_with_results<S>(message: S, results: Changeset)
where
    S: Display,
{
    end_action_internal(Some(message), Some(results));
}

pub fn end_action_without_message() {
    end_action_internal(None::<String>, None);
}

pub fn log_changeset_with_prefix<S>(changeset: Changeset, prefix: S)
where
    S: Display,
{
    log(changeset
        .diffs
        .iter()
        .map(|diff| match diff {
            Difference::Same(same) => with_prefix_and_style(
                same,
                format!("{prefix}{SPACING}{SPACING}"),
                Style::default().dimmed(),
            ),
            Difference::Add(add) => with_prefix_and_style(
                add,
                format!("{prefix}{SPACING}{} ", Paint::green("+")),
                Style::new(Color::Green),
            ),
            Difference::Rem(rem) => with_prefix_and_style(
                rem,
                format!("{prefix}{SPACING}{} ", Paint::red("-")),
                Style::new(Color::Red),
            ),
        })
        .collect::<Vec<String>>()
        .join(&changeset.split));
}

pub fn log_changeset(changeset: Changeset) {
    log_changeset_with_prefix(changeset, "");
}
