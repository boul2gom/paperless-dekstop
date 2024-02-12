import { Carousel as MantineCarousel } from '@mantine/carousel';
import { Paper, Text, Title, Button } from '@mantine/core';
import classes from '@/src/styles/Content.module.css';
import { fetch_backend, invoke_backend } from './Utils/Utils';
import Error from 'next/error';
import useSWR from 'swr';

interface CardProperties {
  id: number;
  image: string;
  title: string;
  category: string;
}

function Card({ image, title, category }: CardProperties) {
  return (
    <Paper
      shadow="md"
      p="xl"
      radius="md"
      style={{ backgroundImage: `url(${image})` }}
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

export function CarouselSkeleton() {
  return (
    <>
      <div>
        Loading...
      </div>
    </>
  );
} 

export function Carousel() {
  const { data, error } = useSWR('favourites', () => fetch_backend<number[]>('Carousel', 'get_favourites'));

  if (error) return <Error statusCode={500} title='Internal Server Error' />;
  if (!data) { console.log('No data were found...'); return <CarouselSkeleton />; }

  return (
    <>
      <MantineCarousel slideSize="30%" slideGap="md" loop withIndicators>
        {data.map((id) => (
          <Card id={id} key={id} image="https://images.unsplash.com/photo-1706554596177-35b0a05a082e" title={`Document ${id}`} category="Category" />
        ))}
      </MantineCarousel>

      <Button onClick={() => invoke_backend("Carousel", "add_to_favourites", { id: 1 })}>Add to favourites</Button>
    </>
  );
}