// This file has been generated by Specta. DO NOT EDIT.

export type ServerMessage = { GameState: GameState }

export type TeamKind = "MrX" | "Detective" | "Observer"

export type Team = { id: number; name: string; color: string; kind: TeamKind }

export type ClientMessage = { Position: { long: number; lat: number } } | { SetTeamPosition: { long: number; lat: number } } | { JoinTeam: { team_id: number } } | { EmbarkTrain: { train_id: string } } | "DisembarkTrain" | { Message: string }

export type CreateTeamError = "InvalidName" | "NameAlreadyExists"

export type Train = { id: number; long: number; lat: number; line_id: string; line_name: string; direction: string }

export type TeamState = { team: Team; long: number; lat: number; on_train: string | null }

export type CreateTeam = { name: string; color: string; kind: TeamKind }

export type GameState = { teams: TeamState[]; trains: Train[] }

/**
 * Information about a tram station.
 */
export type Stop = { name: string; id: string; lat: number; lon: number }

