import { css } from "./styled-system/css/index.mjs";

const gray = "#949494";
const reset = { margin: "0", padding: "0", listStyle: "none" } as const;

export const classes = {
  app: css({
    background: "#fff",
    maxWidth: "550px",
    margin: "130px auto 40px",
    position: "relative",
    boxShadow: "0 2px 4px rgba(0,0,0,.2), 0 25px 50px rgba(0,0,0,.1)",
  }),
  heading: css({
    position: "absolute",
    top: "-110px",
    width: "100%",
    fontSize: "80px",
    fontWeight: "200",
    textAlign: "center",
    color: "#b83f45",
    margin: "0",
  }),
  newTodo: css({ boxShadow: "inset 0 -2px 1px rgba(0,0,0,.03)" }),
  main: css({ position: "relative", borderTop: "1px solid #e6e6e6" }),
  list: css(reset),
  item: `group ${css({
    position: "relative",
    fontSize: "24px",
    borderBottom: "1px solid #ededed",
    display: "flex",
    alignItems: "center",
  })}`,
  itemDone: "",
  toggle: css({ width: "40px", height: "40px", margin: "0 10px" }),
  title: css({ flex: "1", padding: "15px", wordBreak: "break-all", transition: "color .2s" }),
  titleDone: css({ color: gray, textDecoration: "line-through" }),
  destroy: css({
    display: "none",
    border: "none",
    background: "none",
    fontSize: "30px",
    color: gray,
    marginRight: "10px",
    cursor: "pointer",
    _after: { content: '"×"' },
    _groupHover: { display: "block" },
  }),
  footer: css({
    padding: "10px 15px",
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    borderTop: "1px solid #e6e6e6",
    color: "#111",
  }),
  filters: css(reset, { display: "flex", gap: "4px" }),
  filterLink: css({
    color: "inherit",
    textDecoration: "none",
    padding: "3px 7px",
    border: "1px solid transparent",
    borderRadius: "3px",
  }),
  selected: css({ borderColor: "#ce4646" }),
};
