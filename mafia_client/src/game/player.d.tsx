
export default interface Player {
    name: string,
    index: number | undefined
    buttons: {
        dayTarget: boolean,
        target: boolean,
        vote: boolean,
    },
    numVoted: number | null,
    alive: boolean,

    getString(): string
}