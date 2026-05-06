export type AmbientSoundKind = "rain" | "wind";

const ambientSoundSources: Record<AmbientSoundKind, string> = {
  rain: "/sounds/rain.mp3",
  wind: "/sounds/wind.mp3",
};

export class AmbientSoundPlayer {
  private audio: HTMLAudioElement | null = null;

  async start(kind: AmbientSoundKind, volume: number) {
    this.stop();

    const audio = new Audio(ambientSoundSources[kind]);
    audio.loop = true;
    audio.volume = normalizeVolume(volume);
    this.audio = audio;

    try {
      await audio.play();
    } catch (error) {
      this.stop();
      throw error;
    }
  }

  stop() {
    if (this.audio === null) {
      return;
    }

    this.audio.pause();
    this.audio.currentTime = 0;
    this.audio = null;
  }

  setVolume(volume: number) {
    if (this.audio === null) {
      return;
    }

    this.audio.volume = normalizeVolume(volume);
  }
}

function normalizeVolume(volume: number) {
  return Math.min(Math.max(volume, 0), 1);
}
