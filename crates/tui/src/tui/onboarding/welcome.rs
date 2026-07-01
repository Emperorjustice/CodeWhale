//! Welcome screen content for onboarding.

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::palette;

pub fn lines() -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled(
            "codewhale",
            Style::default()
                .fg(palette::WHALE_ACCENT_PRIMARY)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("Version {}", env!("CARGO_PKG_VERSION")),
            Style::default().fg(palette::TEXT_MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Make CodeWhale yours before the first prompt.",
            Style::default().fg(palette::TEXT_PRIMARY),
        )),
        Line::from(Span::styled(
            "Setup will choose your language, check provider readiness, and create or confirm your CodeWhale constitution.",
            Style::default().fg(palette::TEXT_MUTED),
        )),
        Line::from(Span::styled(
            "Bundled defaults are valid; you can personalize standing guidance now or come back later with /constitution.",
            Style::default().fg(palette::TEXT_MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to continue.",
            Style::default().fg(palette::TEXT_PRIMARY),
        )),
        Line::from(Span::styled(
            "Ctrl+C exits at any point.",
            Style::default().fg(palette::TEXT_MUTED),
        )),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body() -> String {
        lines()
            .into_iter()
            .flat_map(|line| {
                line.spans
                    .into_iter()
                    .map(|span| span.content.to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn welcome_copy_centers_constitution_first_setup() {
        let body = body();

        assert!(body.contains("CodeWhale constitution"));
        assert!(body.contains("provider readiness"));
        assert!(body.contains("/constitution"));
        assert!(!body.contains("add an API key"));
        assert!(!body.contains("land in the chat"));
    }
}
