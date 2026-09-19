export interface GameResponse {
  message: string;
  location: string;
  inventory: string[];
  turn: number;
  available_exits: string[];
}

const api = 'http://127.0.0.1:5000/api/game';

export async function newGame(): Promise<GameResponse> {
  return fetch(`${api}/new`, { method: 'POST' }).then((r) => r.json());
}

export async function sendCommand(command: string): Promise<GameResponse> {
  const response = await fetch(`${api}/command`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ command }) });
  if (!response.ok) throw new Error(await response.text());
  return response.json();
}
