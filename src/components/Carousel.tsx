import { Carousel as MantineCarousel } from '@mantine/carousel';
import { Paper, Text, Title, Button } from '@mantine/core';
import classes from '@/styles/Content.module.css';
import React, { Suspense } from 'react';
import {useSuspenseQueries, useSuspenseQuery} from "@tanstack/react-query";
import {bytes_to_image} from "@/utils/utils.ts";
import {fetcher, fetcher_with_args} from "@/utils/backend.ts";

interface CardProperties {
  id: number;
  image: string;
  title: string;
  category: string;
}

const Card: React.FC<CardProperties> = ({ image, title, category }) => {
  return (
      <Paper shadow="md" p="xl" radius="md" style={{ backgroundImage: `url(${image})` }} className={classes.carousel_card}>
        <div>
          <Text className={classes.carousel_card_category} size="xs">{category}</Text>
          <Title order={3} className={classes.carousel_card_title}>{title}</Title>
        </div>
        <Button variant="white" color="dark">View document</Button>
      </Paper>
  );
}

const Skeleton = () => {
  const cards = Array.from({ length: 5 }).map((_, index) => (
    <MantineCarousel.Slide key={index}>
      <Paper shadow="md" p="xl" radius="md" className={classes.carousel_card}>
        <div>
          <Text className={classes.carousel_card_category} size="xs">Category</Text>
          <Title order={3} className={classes.carousel_card_title}>Document</Title>
        </div>
        <Button variant="white" color="dark">View is loading...</Button>
      </Paper>
    </MantineCarousel.Slide>
  ));

  return cards;
}

const Cards = () => {
  const { data: favourites } = useSuspenseQuery({
      queryKey: ["get_favourites"],
      queryFn: () => fetcher<number[]>("get_favourites"),
  })

  const queries = useSuspenseQueries({
      queries: favourites.map((id: number) => ({
            queryKey: ["document_thumbnail", id],
            queryFn: () => fetcher_with_args<Uint8Array>("document_thumbnail", { id }),
        })),
  });

  const cards = queries.map((result, id) => {
      const image = bytes_to_image(result.data, "webp");

      return (
          <MantineCarousel.Slide key={id}>
              <Card id={id} key={id} image={image} title={`Document ${id}`} category="Category" />
          </MantineCarousel.Slide>
      )
  });

  return cards;
}

export const Carousel = () => {
  return (
      <MantineCarousel height="100%" slideGap="md" loop withIndicators align="center" classNames={{
            root: classes.carousel,
      }} slidesToScroll={2} slideSize="50%">
        <Suspense fallback={<Skeleton />}>
          <Cards />
        </Suspense>
      </MantineCarousel>
  );
}