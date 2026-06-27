export class ColorPaletteGenerator {
  private readonly knownKeys: Map<string, number> = new Map();

  constructor(
    private readonly maxKeys: number,
    // @ts-expect-error unused
    private readonly overflowHueShift: number = 0,
  ) {}

  private formatHSL(hue: number): string {
    return `hsl(${hue}, 100%, 50%)`;
  }

  getColor(key: string): string {
    if (this.knownKeys.has(key)) {
      return this.formatHSL(this.knownKeys.get(key)!);
    }

    const frac = (this.knownKeys.size % this.maxKeys) / this.maxKeys;
    const hue = frac * 360;

    this.knownKeys.set(key, hue);
    return this.formatHSL(hue);
  }

  reset(): void {
    this.knownKeys.clear();
  }
}
