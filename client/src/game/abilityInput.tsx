type AbilityInput = {
    type: "auditor",
    input: [number | null, number | null]
} | {
    type: "ojoInvestigate",
    input: [number | null, number | null]
}