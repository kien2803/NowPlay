export interface TrackMetadata {
  title?: string;
  artist?: string;
  album?: string;
  artUrl?: string;
  lengthMs?: number;
}

export interface PlayerState {
  playerId: string;
  playerName: string;
  status: "playing" | "paused" | "stopped";
  metadata: TrackMetadata;
  positionMs: number;
  canControl: boolean;
  canSeek: boolean;
  volume: number | null;
}
