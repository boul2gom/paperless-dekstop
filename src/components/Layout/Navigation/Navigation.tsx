import { useState } from "react";
import { NavigationCategory, toggle_category } from "@/src/components/Layout/Navigation/NavigationCategory";

import classes from "@/src/styles/Layout.module.css";
import { IconGauge, IconFolderSearch, IconFileDescription, IconFolderCog, IconAdjustments } from "@tabler/icons-react";

const main_categories = [
  { label: "Dashboard", icon: IconGauge },
  { label: "Files repository", icon: IconFolderSearch },
];

const linked_categories = [
  {
    label: "Documents", icon: IconFileDescription, initially_opened: true,
    links: [
      { label: "Tags", link: "/" },
      { label: "Correspondents", link: "/" },
      { label: "Document types", link: "/" },
    ],
  },
  {
    label: "Management", icon: IconFolderCog, initially_opened: false,
    links: [
      { label: "Mail", link: "/" },
      { label: "Workflows", link: "/" },
      { label: "Storage paths", link: "/" },
      { label: "Custom fields", link: "/" },
    ],
  },
  {
    label: "Administration", icon: IconAdjustments, initially_opened: false,
    links: [
      { label: "Settings", link: "/" },
      { label: "Tasks & logs", link: "/" },
      { label: "Users & groups", link: "/" },
    ],
  }
];

export function Navigation() {
  const [categories_opened, set_categories_opened] = useState(
      linked_categories.map((category) => category.initially_opened || false)
  );

  const categories = main_categories.map((item) => (
          <NavigationCategory is_open={false} on_toggle={() => {}} key={item.label} {...item} />
      )).concat(
          linked_categories.map((item, index) => (
            <NavigationCategory
              {...item}
              key={item.label}
              is_open={categories_opened[index]}
              on_toggle={() => toggle_category(categories_opened, set_categories_opened, index)}
            />
          ))
  );

  return (
      <nav className={classes.navigation}>
        <div className={classes.category_links_inner}>
          {categories.map((category) => <div key={category.key}>{category}</div>)}
        </div>
      </nav>
  );
}