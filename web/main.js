class Tower {
    constructor(tower, range) {
        this.pos_x = tower.position.x;
        this.pos_y = tower.position.y;
        this.negotiable = true;
        this.buffable = true;
        this.jitsu_value = 9999999;
        this.shinobi_value = 9999999;
        this.range = range;
    }
    into_nonnegotiable() {
        this.negotiable = false;
        return this;
    }
    into_unbuffable() {
        this.buffable = false;
        return this;
    }
    with_values(jitsu, shinobi) {
        this.jitsu_value = jitsu;
        this.shinobi_value = shinobi;
        return this;
    }
}

function tower_is_in_range(t1, t2, r) {
    let dx = t1.pos_x - t2.pos_x;
    let dy = t1.pos_y - t2.pos_y;
    return Math.sqrt(dx * dx + dy * dy) < r;
}

function get_attack_speed_buff(stack_count) {
    return Math.pow(1.08, stack_count);
}

function get_pierce_buff(stack_count) {
    return 1 + 0.08 * stack_count;
}

function get_setup_dps(towers, types, stacks) {
    let dps = 0;
    for (let i = 0; i < towers.length; i++) {
        const tower = towers[i];
        if (tower.negotiable) {
            const type = types[i];
            const stack_count = Math.min(20, stacks[i]);
            const value = type == TOWER_TYPES.Shinobi ? tower.shinobi_value : tower.jitsu_value;
            const attack_speed_buff = get_attack_speed_buff(stack_count);
            const pierce_buff = get_pierce_buff(stack_count);
            dps += value * attack_speed_buff * pierce_buff;
        }
    }
    return dps;
}

function optimize(towers, types, stacks, inrange_lookup, depth = 1) {
    let best_improvement = [get_setup_dps(towers, types, stacks), structuredClone(types), structuredClone(stacks)];
    for (let i = 0; i < towers.length; i++) {
        const tower = towers[i];
        if (!tower.negotiable) {
            continue;
        }
        switch (types[i]) {
            case TOWER_TYPES.Jitsu: {
                let new_types = structuredClone(types);
                let new_stacks = structuredClone(stacks);
                new_types[i] = TOWER_TYPES.Shinobi;
                inrange_lookup[i].forEach(other => {
                    new_stacks[other] += 1;
                });
                if (depth == 1) {
                    const dps_change = get_setup_dps(towers, new_types, new_stacks);
                    if (dps_change > best_improvement[0]) {
                        best_improvement = [dps_change, new_types, new_stacks];
                    }
                } else {
                    const best_step = optimize(towers, new_types, new_stacks, inrange_lookup, depth - 1);
                    if (best_step[0] > best_improvement[0]) {
                        best_improvement = [best_step[0], best_step[1], best_step[2]];
                    }
                }
            } break;
            case TOWER_TYPES.Shinobi: {
                let new_types = structuredClone(types);
                let new_stacks = structuredClone(stacks);
                new_types[i] = TOWER_TYPES.Jitsu;
                inrange_lookup[i].forEach(other => {
                    new_stacks[other] -= 1;
                });
                if (depth == 1) {
                    const dps_change = get_setup_dps(towers, new_types, new_stacks);
                    if (dps_change > best_improvement[0]) {
                        best_improvement = [dps_change, new_types, new_stacks];
                    }
                } else {
                    const best_step = optimize(towers, new_types, new_stacks, inrange_lookup, depth - 1);
                    if (best_step[0] > best_improvement[0]) {
                        best_improvement = [best_step[0], best_step[1], best_step[2]];
                    }
                }
            } break;
        }
    }
    return best_improvement;
}

const TOWER_TYPES = Object.freeze({
    Other: 0,
    Jitsu: 1,
    Shinobi: 2,
});

const MAX_RANGE = 84.755;
const RANGE_NO_VILLAGE = 77.05;
const NINJAGON_RANGE = 70.0;
const VILLAGE_RANGE = 136.65420718284065;
const HOMELAND_RANGE = 77;

import POPS_JITSU from "./Logs_FreeplaySetupPops.json" with { type: 'json' };
import POPS_SHINOBI from "./Logs_FreeplaySetupPopsShi.json" with { type: 'json' };
import SETUP from "./Logs_FreeplaySetup.json" with { type: 'json' };

const JITSU_POP_LOOKUP = Object.fromEntries(
    POPS_JITSU.placedTowers.map((t) => [t.IdLastSave, t.damageDealt]),
);
const SHINOBI_POP_LOOKUP = Object.fromEntries(
    POPS_SHINOBI.placedTowers.map((t) => [t.IdLastSave, t.damageDealt]),
);

const VILLAGES = SETUP.placedTowers
    .filter((t) => t.baseId === "MonkeyVillage")
    .map((t) =>
        new Tower(t, t.pathOneTier == 5 ? VILLAGE_RANGE : HOMELAND_RANGE)
            .into_nonnegotiable()
            .into_unbuffable(),
    );

const towers = [];
let types = [];

SETUP.placedTowers.forEach((t) => {
    if (t.baseId === "NinjaMonkey") {
        const jitsu_value = JITSU_POP_LOOKUP[t.IdLastSave];
        const shinobi_value = SHINOBI_POP_LOOKUP[t.IdLastSave];
        if (t.pathOneTier === 6) {
            types.push(TOWER_TYPES.Shinobi);
            towers.push(
                new Tower(t, NINJAGON_RANGE)
                    .into_nonnegotiable()
                    .into_unbuffable(),
            );
        } else if (4 <= t.pathOneTier && t.pathOneTier <= 5) {
            let jitsu = new Tower(t, 0.0).with_values(
                jitsu_value,
                shinobi_value,
            );
            let range = RANGE_NO_VILLAGE;
            for (let i = 0; i < VILLAGES.length; i++) {
                const v = VILLAGES[i];
                if (tower_is_in_range(jitsu, v, v.range)) {
                    range = MAX_RANGE;
                    break;
                }
            }
            jitsu.range = range;
            types.push(TOWER_TYPES.Jitsu);
            towers.push(jitsu);
        } else if (t.pathTwoTier >= 3) {
            let shinobi = new Tower(t, 0.0).with_values(
                jitsu_value,
                shinobi_value,
            );
            let range = RANGE_NO_VILLAGE;
            for (let i = 0; i < VILLAGES.length; i++) {
                const v = VILLAGES[i];
                if (tower_is_in_range(shinobi, v, v.range)) {
                    range = MAX_RANGE;
                    break;
                }
            }
            shinobi.range = range;
            types.push(TOWER_TYPES.Shinobi);
            towers.push(shinobi);
        } 
    } else if (t.baseId === "MonkeyVillage") {
        let range = t.pathOneTier == 5 ?  VILLAGE_RANGE : HOMELAND_RANGE;
        types.push(TOWER_TYPES.Other);
        towers.push(new Tower(t, range).into_nonnegotiable().into_unbuffable());
    }
});

let stacks = [];
let inrange_lookup = [];

for (let i = 0; i < towers.length; i++) {
    stacks.push(0);
}

for (let i = 0; i < towers.length; i++) {
    inrange_lookup.push([]);
    for (let j = 0; j < towers.length; j++) {
        if (tower_is_in_range(towers[j], towers[i], towers[i].range)) {
            inrange_lookup[i].push(j);
            if (types[i] === TOWER_TYPES.Shinobi) {
                stacks[j] += 1;
            }
        }
    }
}

let dps = get_setup_dps(towers, types, stacks);
console.log(`inital dps: ${dps}`);
for (let i = 1;;i++) {
    let improvement = optimize(towers, types, stacks, inrange_lookup, 2);
    if (improvement[0] - dps < 0.1) {
        break;
    }
    dps = improvement[0];
    types = improvement[1];
    stacks = improvement[2];
    console.log(`${i} step optimized dps: ${get_setup_dps(towers, types, stacks)}`);
}
