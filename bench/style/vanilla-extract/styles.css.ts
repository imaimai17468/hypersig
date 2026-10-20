import { globalStyle, style } from "@vanilla-extract/css";

const gray = "#949494";

globalStyle("*", { boxSizing: "border-box" });
globalStyle("body", {
  font: "14px 'Helvetica Neue', Helvetica, Arial, sans-serif",
  background: "#f5f5f5",
  color: "#111",
  margin: 0,
});

const reset = style({ margin: 0, padding: 0, listStyle: "none" });

export const app = style({
  background: "#fff",
  maxWidth: 550,
  margin: "130px auto 40px",
  position: "relative",
  boxShadow: "0 2px 4px rgba(0,0,0,.2), 0 25px 50px rgba(0,0,0,.1)",
});
export const heading = style({
  position: "absolute",
  top: -110,
  width: "100%",
  fontSize: 80,
  fontWeight: 200,
  textAlign: "center",
  color: "#b83f45",
  margin: 0,
});
export const newTodo = style({ boxShadow: "inset 0 -2px 1px rgba(0,0,0,.03)" });
export const main = style({ position: "relative", borderTop: "1px solid #e6e6e6" });
export const list = reset;
export const item = style({
  position: "relative",
  fontSize: 24,
  borderBottom: "1px solid #ededed",
  display: "flex",
  alignItems: "center",
});
export const toggle = style({ width: 40, height: 40, margin: "0 10px" });
export const title = style({ flex: 1, padding: 15, wordBreak: "break-all", transition: "color .2s" });
export const titleDone = style({ color: gray, textDecoration: "line-through" });
export const destroy = style({
  display: "none",
  border: "none",
  background: "none",
  fontSize: 30,
  color: gray,
  marginRight: 10,
  cursor: "pointer",
  "::after": { content: '"×"' },
  selectors: { [`${item}:hover &`]: { display: "block" } },
});
export const footer = style({
  padding: "10px 15px",
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  borderTop: "1px solid #e6e6e6",
  color: "#111",
});
export const filters = style([reset, { display: "flex", gap: 4 }]);
export const filterLink = style({
  color: "inherit",
  textDecoration: "none",
  padding: "3px 7px",
  border: "1px solid transparent",
  borderRadius: 3,
});
export const selected = style({ borderColor: "#ce4646" });
