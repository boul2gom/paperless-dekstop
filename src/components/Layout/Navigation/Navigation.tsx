import { Suspense, useState } from "react";
import { NavigationCategory, toggle_category } from "@/components/Layout/Navigation/NavigationCategory";

import classes from "@/styles/Layout.module.css";
import { IconGauge, IconFolderSearch, IconFileDescription, IconFolderCog, IconAdjustments } from "@tabler/icons-react";
import { Code, Group, Skeleton, Image } from "@mantine/core";
import { useSuspenseQuery } from "@tanstack/react-query";
import { fetcher } from "@/components/Utils/Utils";

import logo from "@/assets/paperless-ngx.png";

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

const ReleaseSkeleton = () => { 
  return (<Skeleton radius="xs" className={classes.release_block} />);
};

const ReleaseBlock = () => {
  const { data } = useSuspenseQuery({
      queryKey: ["latest_release"],
      queryFn: () => fetcher<string>("latest_release"),
  });

  return (
      <Code fw={700} className={classes.release_block}>{data}</Code>
  );
}

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
      <>
        <Group justify="space-between" className={classes.logo}>
          <Image src={logo} alt="Paperless Logo" width={159} height={60} />
          <Suspense fallback={<ReleaseSkeleton />}>
            <ReleaseBlock />
          </Suspense>
        </Group>

        <nav className={classes.navigation}>
          <div className={classes.category_links_inner}>
            {categories.map((category) => <div key={category.key}>{category}</div>)}
          </div>
        </nav>
      </>
  );
}