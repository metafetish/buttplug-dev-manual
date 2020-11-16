let date = new Date().toJSON();
// .vuepress/config.js
module.exports = {
  themeConfig: {
    sidebar: [
      "/",
      "/foreword.md",
      {
        title: "Flared Basics",
        collapsable: true,
        children: [
          "/intro/introduction.md",
          "/intro/how-to-read.md",
          "/intro/buttplug-ethics.md",
        ]
      },
      {
        title: "Strategies Against Buttplug Architecture",
        collapsable: true,
        children: [
          "/architecture/intro.md",
          "/architecture/the-3-genders.md",
          "/architecture/protocol-in-depth.md",
          "/architecture/server-in-depth.md",
          "/architecture/client-in-depth.md",
          "/architecture/terms.md"
        ]
      },
      /*
      {
        title: "Sticking Buttplug In",
        collapsable: true,
        children: [
          "/writing-buttplug-applications/intro.md",
          "/writing-buttplug-applications/api-basics.md",
          "/writing-buttplug-applications/connectors.md",
          "/writing-buttplug-applications/connecting.md",
          "/writing-buttplug-applications/logging.md",
          "/writing-buttplug-applications/device-enum.md",
          "/writing-buttplug-applications/device-control.md",
          "/writing-buttplug-applications/application.md",
        ]
      },
      {
        title: "Winning Ways For Your Buttplug Plays",
        collapsable: true,
        children: [
          "/extending-buttplug-core/intro.md"
        ]
      },
      {
        title: "Inflating Buttplug",
        collapsable: true,
        children: [
          "/extending-buttplug-core/intro.md"
        ]
      },
      */
    ],
    repo: 'buttplugio/buttplug-developer-guide',
  },
  plugins: [
    [
      "vuepress-plugin-matomo",
      {
        'siteId': 9,
        'trackerUrl': "https://matomo.nonpolynomial.com/"
      }
    ],
    "@vuepress/plugin-back-to-top",
    [
      "vuepress-plugin-container",
      {
        type: 'callout',
        before: info => `<div class="callout"><p class="title">${info}</p>`,
        after: '</div>',
      },
    ],
    "code-switcher"
  ],
  evergreen: true,
  title: "Buttplug Developer Guide",
  description: "Manual for developing applications using the Buttplug Intimate Hardware Control Library.",
  head: [
    ['link', { rel: 'icon', href: '/buttplug.svg' }],
    ["meta", {property: "og:type", content:"website"}],
    ["meta", {property: "og:title", content:"Buttplug Developer Guide"}],
    ["meta", {property: "og:url", content:"https://buttplug-developer-guide.docs.buttplug.io"}],
    ["meta", {property: "og:site_name", content:"Buttplug Developer Guide"}],
    ["meta", {property: "og:description", content:"Manual for developing applications using the Buttplug Intimate Hardware Control Library."}],
    ["meta", {property: "og:locale", content:"default"}],
    ["meta", {property: "og:image", content:"https://buttplug-developer-guide.docs.buttplug.io/buttplug-logo-opengraph.png"}],
    ["meta", {property: "og:updated_time", content:date}],
    ["meta", {name:"twitter:card", content:"summary"}],
    ["meta", {name:"twitter:title", content:"Buttplug Developer Guide"}],
    ["meta", {name:"twitter:description", content:"Manual for developing applications using the Buttplug Intimate Hardware Control Library."}],
    ["meta", {name:"twitter:image", content:"https://buttplug-developer-guide.docs.buttplug.io/buttplug-logo-opengraph.png"}],
    ["meta", {name:"twitter:creator", content:"@buttplugio"}],
  ]
};
