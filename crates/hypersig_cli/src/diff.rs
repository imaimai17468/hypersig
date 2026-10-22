//! The line diff `add` prints before it rewrites `Cargo.toml`.

/// `before` and `after` line by line, each line marked `-` where only
/// `before` has it, `+` where only `after` has it, and a space where both do.
///
/// The longest common subsequence takes time and memory proportional to the
/// product of the two line counts, which a `Cargo.toml` keeps small.
pub fn lines(before: &str, after: &str) -> String {
    let old: Vec<&str> = before.lines().collect();
    let new: Vec<&str> = after.lines().collect();
    // common[i][j] is the length of the longest common subsequence of
    // old[i..] and new[j..].
    let mut common = vec![vec![0_usize; new.len() + 1]; old.len() + 1];
    for i in (0..old.len()).rev() {
        for j in (0..new.len()).rev() {
            common[i][j] = if old[i] == new[j] {
                common[i + 1][j + 1] + 1
            } else {
                common[i + 1][j].max(common[i][j + 1])
            };
        }
    }
    let mut out = String::new();
    let (mut i, mut j) = (0, 0);
    while i < old.len() || j < new.len() {
        let line = if i < old.len() && j < new.len() && old[i] == new[j] {
            i += 1;
            j += 1;
            format!("  {}", old[i - 1])
        } else if i < old.len() && (j == new.len() || common[i + 1][j] >= common[i][j + 1]) {
            i += 1;
            format!("- {}", old[i - 1])
        } else {
            j += 1;
            format!("+ {}", new[j - 1])
        };
        out.push_str(line.trim_end());
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::lines;

    #[test]
    fn marks_removed_and_added_lines() {
        assert_eq!(lines("a\nb\nc\n", "a\nc\nd\n"), "  a\n- b\n  c\n+ d\n");
    }
}
