// This file has been generated by Specta. DO NOT EDIT.

export type Train = { id: number; long: number; lat: number; line_id: string; line_name: string; direction: string }

export type TeamKind = "MrX" | "Detective" | "Observer"

export type GameState = { teams: Team[]; trains: Train[] }

export type Team = { id: number; long: number; lat: number; on_train: string | null; name: string; color: string; kind: TeamKind }

export type CreateTeam = { name: string; color: string; kind: TeamKind }

export type ClientMessage = { Position: { long: number; lat: number } } | { SetTeamPosition: { long: number; lat: number; team_id: number } } | { JoinTeam: { team_id: number } } | { EmbarkTrain: { train_id: string } } | { DisembarkTrain: number } | { Message: string }

/**
 * Information about a tram station
 */
export type Stop = { name: string; id: string; lat: number; lon: number }

