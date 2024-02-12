import { fetch_backend } from '@/src/components/Utils/Utils'
import type { NextApiRequest, NextApiResponse } from 'next'

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  const favourites = await fetch_backend("Carousel", "get_favourites");
  
  res.status(200).json(favourites)
}