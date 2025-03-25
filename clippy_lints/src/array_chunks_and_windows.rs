use clippy_utils::consts::Constant;
use clippy_utils::diagnostics::span_lint;
use clippy_utils::source::SpanRangeExt;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    /// The lint suggests using `array_windows`  and `array_chunks` instead of `windows` and `chunks`
    /// when a constant value is specified. This allows for a more performant execution.
    /// ### Why is this bad?
    /// It adds unnecessary heap allocation.
    /// ### Example
    /// ```no_run
    /// for window in slice.windows(2) {
    ///     println!("{} {}", w[0], w[1]);
    /// }
    /// ```
    /// Use instead:
    /// ```no_run
    /// for [a, b] in slice.array_windows() {
    ///     println!("{} {}", a, b);
    /// }
    /// ```
    #[clippy::version = "1.87.0"]
    pub ARRAY_CHUNKS_AND_WINDOWS,
    nursery,
    "default lint description"
}

declare_lint_pass!(ArrayChunksAndWindows => [ARRAY_CHUNKS_AND_WINDOWS]);

impl LateLintPass<'_> for ArrayChunksAndWindows {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        if let ExprKind::MethodCall(name, _, args, span) = expr.kind
            && (name.ident.as_str() == "windows" || name.ident.as_str() == "chunks")
            && let [expr] = args
        {
            if let ExprKind::Lit(l) = clippy_utils::expr_or_init(cx, expr).kind {
                let arg = clippy_utils::consts::lit_to_mir_constant(&l.node, None);
                if let Constant::Int(n) = arg
                    && n <= 26
                {
                    span_lint(
                        cx,
                        ARRAY_CHUNKS_AND_WINDOWS,
                        span,
                        format!(
                            "use .array_{name}() instead of .{name}({expr})",
                            name = name.ident.as_str(),
                            expr = expr.span.get_source_text(cx).unwrap()
                        ),
                    )
                }
            }
        }
    }
}
