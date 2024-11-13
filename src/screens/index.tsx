import { useState } from "react";
import {
  Snowflake,
  FishIcon,
  Weight,
  Clock,
  Home,
  Utensils,
  Ruler,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { invoke } from "@tauri-apps/api";

interface Fish {
  id: number;
  name: string;
  average_size: string;
  average_weight: string;
  average_lifespan: string;
  habitat: string;
  diet: string;
  blurb: string;
  image_path: string;
}

async function getRandomFish(): Promise<Fish> {
  const fish = await invoke("get_random_fish");

  console.log("Fetched fish:", fish);

  return fish as Fish;
}

function DefaultScreen({ onDiscoverClick }: { onDiscoverClick: () => void }) {
  return (
    <div className="flex flex-col items-center justify-center h-full space-y-8">
      <h2 className="text-4xl font-bold text-blue-700">
        Welcome to Arctic Ice Fishing
      </h2>
      <p className="text-xl text-gray-600 text-center max-w-2xl">
        Dive into the fascinating world of arctic fish! Catch a fish, scan it,
        and uncover fascinating facts about your unique catch.
      </p>
      <Button
        onClick={onDiscoverClick}
        className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-3 px-6 rounded-full shadow-lg transform transition hover:scale-105"
      >
        Scan Placeholder
      </Button>
    </div>
  );
}

function FishDisplay({ fish }: { fish: Fish }) {
  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div className="space-y-6">
        <h2 className="text-3xl font-bold text-blue-700 border-b-2 border-blue-300 pb-2">
          {fish.name}
        </h2>
        <div className="bg-blue-200 p-4 rounded-lg shadow-inner">
          <p className="text-sm text-gray-600 italic">{fish.blurb}</p>
        </div>
        <div className="grid grid-cols-2 gap-4">
          <InfoCard
            icon={<Ruler className="w-5 h-5" />}
            title="Size"
            value={fish.average_size}
          />
          <InfoCard
            icon={<Weight className="w-5 h-5" />}
            title="Weight"
            value={fish.average_weight}
          />
          <InfoCard
            icon={<Clock className="w-5 h-5" />}
            title="Lifespan"
            value={fish.average_lifespan}
          />
          <InfoCard
            icon={<Home className="w-5 h-5" />}
            title="Habitat"
            value={fish.habitat}
          />
          <InfoCard
            icon={<Utensils className="w-5 h-5" />}
            title="Diet"
            value={fish.diet}
            className="col-span-2"
          />
        </div>
      </div>
      <div className="flex flex-col justify-center items-center space-y-4">
        <div className="w-full h-80 bg-blue-100 rounded-lg overflow-hidden relative shadow-lg">
          <img
            src={fish.image_path}
            alt={fish.name}
            className="w-full h-full object-cover"
          />
          <div className="absolute inset-0 bg-gradient-to-t from-blue-500/50 to-transparent"></div>
        </div>
      </div>
    </div>
  );
}

function InfoCard({
  icon,
  title,
  value,
  className = "",
}: {
  icon: JSX.Element;
  title: string;
  value: string;
  className?: string;
}) {
  return (
    <div
      className={`bg-gray-300/50 p-3 rounded-lg shadow flex items-start space-x-3 ${className}`}
    >
      <div className="bg-blue-300 p-2 rounded-full">{icon}</div>
      <div>
        <h3 className="font-semibold text-blue-800">{title}</h3>
        <p className="text-sm text-gray-600">{value}</p>
      </div>
    </div>
  );
}

export function Index() {
  const [fish, setFish] = useState<Fish | null>(null);
  const [loading, setLoading] = useState(false);
  const [showFish, setShowFish] = useState(false);

  const fetchRandomFish = async () => {
    setLoading(true);
    try {
      const newFish = await getRandomFish();
      setFish(newFish);
      setShowFish(true);
      setTimeout(() => setShowFish(false), 10000); // Switch back to default screen after 20 seconds
    } catch (error) {
      console.error("Failed to fetch fish:", error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-blue-100 to-blue-200 flex flex-col justify-center items-center p-4">
      <Card className="w-full max-w-4xl bg-white/80 backdrop-blur-sm shadow-lg rounded-lg overflow-hidden border-4 border-blue-300">
        <CardHeader className="bg-blue-500 text-white">
          <CardTitle className="text-3xl font-bold flex items-center justify-between">
            <span>Arctic Fish Explorer</span>
            <Snowflake className="w-8 h-8" />
          </CardTitle>
        </CardHeader>
        <CardContent className="p-6">
          {loading ? (
            <div className="flex justify-center items-center h-64">
              <FishIcon className="w-16 h-16 text-blue-500 animate-bounce" />
            </div>
          ) : showFish && fish ? (
            <FishDisplay fish={fish} />
          ) : (
            <DefaultScreen onDiscoverClick={fetchRandomFish} />
          )}
        </CardContent>
      </Card>
      <div className="fixed top-0 left-0 right-0 bottom-0 pointer-events-none">
        {[...Array(30)].map((_, i) => (
          <Snowflake
            key={i}
            className="text-white opacity-50 absolute animate-fall"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
              animationDuration: `${Math.random() * 10 + 5}s`,
              animationDelay: `${Math.random() * 5}s`,
              fontSize: `${Math.random() * 10 + 10}px`,
            }}
          />
        ))}
      </div>
    </div>
  );
}
