/**
 * Replaces each `<template>` below `root` that the parser read as an SVG
 * element with an HTML `<template>` holding the same attributes and children.
 *
 * Inside `<svg>`, the HTML parser makes `<template>` an SVG element whose
 * children are live, unrendered SVG elements, so a list or a condition there
 * has no `content` to clone. The children keep the SVG namespace they were
 * parsed in. Descendants come before ancestors, so a nested template is
 * replaced before its ancestor moves it into `content`.
 */
export function upgradeTemplates(root: ParentNode): void {
  // The selector's type says HTML, but it also matches the SVG elements this function exists for.
  const found = Array.from(root.querySelectorAll<Element>("template")).filter(
    (element) => !(element instanceof HTMLTemplateElement),
  );
  for (const svg of found.toReversed()) {
    const template = document.createElement("template");
    for (const { name, value } of svg.attributes) template.setAttribute(name, value);
    template.content.append(...svg.childNodes);
    svg.replaceWith(template);
  }
}
