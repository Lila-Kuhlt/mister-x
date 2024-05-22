// This file has been generated by Specta. DO NOT EDIT.

/**
 * Information about a tram station.
 */
export type Stop = { name: string; id: string; lat: number; lon: number }

export type ClientResponse = { GameState: GameState } | { MrXGadget: MrXGadget } | { DetectiveGadget: DetectiveGadget }

export type MrXGadget = { AlternativeFacts: { stop_id: string } } | { Midjourney: { image: number[] } } | "NotFound" | "Teleport" | "Shifter"

export type GameState = { teams: TeamState[]; trains: Train[]; position_cooldown: number | null; detective_gadget_cooldown: number | null; mr_x_gadget_cooldown: number | null }

export type CreateTeam = { name: string; color: string; kind: TeamKind }

export type DetectiveGadget = { Stop: { stop_id: string } } | "OutOfOrder" | "Shackles"

export type CreateTeamError = "InvalidName" | "NameAlreadyExists"

export type TeamKind = "MrX" | "Detective" | "Observer"

export type ClientMessage = { Position: { long: number; lat: number } } | { SetTeamPosition: { long: number; lat: number } } | { JoinTeam: { team_id: number } } | { EmbarkTrain: { train_id: string } } | "DisembarkTrain" | { MrXGadget: MrXGadget } | { DetectiveGadget: DetectiveGadget } | { Message: string }

export type TeamState = { team: Team; long: number; lat: number; on_train: string | null }

export type Train = { id: number; long: number; lat: number; line_id: string; line_name: string; direction: string }

export type Team = { id: number; name: string; color: string; kind: TeamKind }

