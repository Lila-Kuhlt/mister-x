import { Array, Literal, Null, Number, Record, String, Union } from "runtypes";

const TeamKind = Union(
    Literal("MrX"),
    Literal("Detective"),
    Literal("Observer"),
);

const Team = Record({
    id: Number,
    name: String,
    color: String,
    kind: TeamKind,
});

const TeamState = Record({
    team: Team,
    long: Number,
    lat: Number,
    on_train: Union(String, Null),
});

const Train = Record({
    id: Number,
    long: Number,
    lat: Number,
    line_id: String,
    line_name: String,
    direction: String,
});

const GameState = Record({
    teams: Array(TeamState),
    trains: Array(Train),
});

export default {
    GameState,
};
