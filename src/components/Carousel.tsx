import { Carousel as MantineCarousel } from '@mantine/carousel';
import { Paper, Text, Title, Button } from '@mantine/core';
import classes from '@/styles/Content.module.css';
import { Suspense } from 'react';

interface CardProperties {
  id: number;
  image: string;
  title: string;
  category: string;
}

function Card({image, title, category}: Readonly<CardProperties>) {
  return (
      <Paper
          shadow="md"
          p="xl"
          radius="md"
          style={{backgroundImage: `url(${image})`}}
          className={classes.carousel_card}
      >
        <div>
          <Text className={classes.carousel_card_category} size="xs">
            {category}
          </Text>
          <Title order={3} className={classes.carousel_card_title}>
            {title}
          </Title>
        </div>
        <Button variant="white" color="dark">
          View document
        </Button>
      </Paper>
  );
}

const CarouselSkeleton = () => {
  return (
      <div>
        Loading skeleton...
      </div>
  );
}

const CarouselBlock = () => {
  const data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

  const cards = data.map((id) => (
    <Card id={id} key={id} image="https://images.unsplash.com/photo-1706554596177-35b0a05a082e" title={`Document ${id}`} category="Category" />
  ));

  return (
    <div className={classes.carousel_container}>
      {cards}
    </div>
    );
}

export default function Carousel() {
  return (
      <MantineCarousel slideSize="30%" slideGap="md" loop withIndicators>
        <Suspense fallback={<CarouselSkeleton/>}>
          <CarouselBlock/>
        </Suspense>
      </MantineCarousel>
  );
}