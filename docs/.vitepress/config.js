import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Import-meta-env",

  description: "Populates your environment variables after build-time.",

  base: "/import-meta-env/",

  lastUpdated: true,

  themeConfig: {
    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2021-present Ernest",
    },

    nav: nav(),

    sidebar: {
      "/guide/": sidebarGuide(),
    },
  },
});

function nav() {
  return [
    {
      text: "Guide",
      link: "/guide/getting-started/prerequisite.md",
      activeMatch: "/guide/",
    },
    {
      text: "API",
      link: "/api",
    },
    {
      text: "Examples",
      link: "https://github.com/iendeavor/import-meta-env/tree/main/packages/examples",
    },
    {
      text: "GitHub",
      link: "https://github.com/iendeavor/import-meta-env",
    },
  ];
}

function sidebarGuide() {
  return [
    {
      text: "Getting started",
      collapsible: true,
      items: [
        {
          text: "Prerequisite",
          link: "/guide/getting-started/prerequisite.md",
        },
        {
          text: "Installation",
          link: "/guide/getting-started/installation.md",
        },
        { text: "Usage", link: "/guide/getting-started/usage.md" },
      ],
    },
    {
      text: "Guide",
      collapsible: true,
      items: [
        { text: "Extra topic", link: "/guide/extra-topic.md" },
        {
          text: "Framework specific notes",
          link: "/guide/framework-specific-notes.md",
        },
        { text: "FAQ", link: "/guide/faq.md" },
      ],
    },
  ];
}
