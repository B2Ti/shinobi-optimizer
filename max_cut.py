import networkx as nx
from networkx.algorithms.approximation import maxcut
import json
import math

with open("Logs_FreeplaySetup.json", "rb") as f:
    actualSetup = json.load(f)

with open("Logs_FreeplaySetupPops.json", "rb") as f:
    setup = json.load(f)

with open("Logs_FreeplaySetupPopsShi.json", "rb") as f:
    setupShinobis = json.load(f)

with open("Logs_FreeplaySetup.json", "rb") as f:
    setupOld = json.load(f)

for tower in setupOld['placedTowers']:
    if tower['baseId'] == "NinjaMonkey" and tower['pathOneTier'] == 6:
        ninjagon = tower

x=0

maxRange = 84.755
rangeNoVillage = 77.05
ninjagonRange = 70
ninja001Range = 47

villageRange: float | int = 55 #1.1x oc 1.015^10x for uboost, 1.2x 1.2x 1.15x 1.5x
villageRange += (0.1*villageRange) + (((1.015**10)-1)*villageRange) + (0.4*villageRange) + (0.15*villageRange) + (0.5*villageRange)
villageRange = 55 #1.1x oc 1.015^10x for uboost, 1.2x 1.2x 1.15x 1.5x
villageRange += (0.1*villageRange) + (((1.015**10)-1)*villageRange) + (0.4*villageRange) + (0.5*villageRange)
villageRange *= 1.15
homelandRange = 55 + (0.4*55)

Jitsus = []
Shinobis = [{"id": ninjagon['IdLastSave'], "pos": (ninjagon['position']['x'], ninjagon['position']['y']), "negotiable": False, "buffable": False, "stacks": 0, "value": 9999999, "range": ninjagonRange, "fullData": ninjagon}] 
villages = []

def inRange(tower1, tower2, tower2Range):
    """Checks if tower 1 is in the range of tower 2"""
    if 'position' in tower1:
        tower1['pos'] = (tower1['position']['x'], tower1['position']['y'])
    if 'position' in tower2:
        tower2['pos'] = (tower2['position']['x'], tower2['position']['y'])
    
    return math.sqrt(((tower1['pos'][0]-tower2['pos'][0])**2)+((tower1['pos'][1]-tower2['pos'][1])**2)) < tower2Range

towers = setup['placedTowers']
towers2 = setupShinobis['placedTowers']
towersActual = actualSetup['placedTowers']
ninjas = []
ninjas2 = []
ninjasActual = []
for tower in towers:
    if tower['baseId'] == 'NinjaMonkey':
        ninjas.append(tower)

    if tower['baseId'] == "MonkeyVillage":
        if tower['pathOneTier'] == 5:
            villages.append({"id": tower['IdLastSave'], "pos": (tower['position']['x'], tower['position']['y']), "negotiable": False, "buffable": False, "stacks": 0, "value": 9999999, "range": villageRange, "fullData": tower})
        else:
            villages.append({"id": tower['IdLastSave'], "pos": (tower['position']['x'], tower['position']['y']), "negotiable": False, "buffable": False, "stacks": 0, "value": 9999999, "range": homelandRange, "fullData": tower})
for tower in towers2:
    if tower['baseId'] == 'NinjaMonkey':
        ninjas2.append(tower)

for tower in towersActual:
    if tower['baseId'] == 'NinjaMonkey':
        ninjasActual.append(tower)

JitsuIdValueLookup = {}
ShinobiIdValueLookup = {}

jitsuInit = True
normalInit = True

for ninja in ninjas:
    if ninja['pathOneTier'] == 4:
        currNinjaRange = rangeNoVillage
        for village in villages:
            if inRange(ninja, village, village['range']):
                currNinjaRange = maxRange
                break
        JitsuIdValueLookup[ninja['IdLastSave']] = ninja['damageDealt']
        if not normalInit:
            if jitsuInit:
                Jitsus.append({"id": ninja['IdLastSave'], "pos": (ninja['position']['x'], ninja['position']['y']), "negotiable": True, "buffable": True, "stacks": 0,  "value": ninja['damageDealt'], "range": currNinjaRange, "fullData": ninja})
for ninja in ninjas2:
    if ninja['pathTwoTier'] == 2:
        currNinjaRange = rangeNoVillage
        for village in villages:
            if inRange(ninja, village, village['range']):
                currNinjaRange = maxRange
                break
        ShinobiIdValueLookup[ninja['IdLastSave']] = ninja['damageDealt']
        if not normalInit:
            if not jitsuInit:
                Shinobis.append({"id": ninja['IdLastSave'], "pos": (ninja['position']['x'], ninja['position']['y']), "negotiable": True, "buffable": True, "stacks": 0,  "value": ninja['damageDealt'], "range": currNinjaRange, "fullData": ninja})
if normalInit:
    for ninja in ninjasActual:
        if ninja['pathOneTier'] >= 4 and ninja['pathOneTier'] < 6:
            currNinjaRange = rangeNoVillage
            for village in villages:
                if inRange(ninja, village, village['range']):
                    currNinjaRange = maxRange
                    break
            Jitsus.append({"id": ninja['IdLastSave'], "pos": (ninja['position']['x'], ninja['position']['y']), "negotiable": True, "buffable": True, "stacks": 0,  "value": JitsuIdValueLookup[ninja['IdLastSave']], "range": currNinjaRange, "fullData": ninja})
        elif ninja['pathTwoTier'] >= 3:
            currNinjaRange = rangeNoVillage
            for village in villages:
                if inRange(ninja, village, village['range']):
                    currNinjaRange = maxRange
                    break
            Shinobis.append({"id": ninja['IdLastSave'], "pos": (ninja['position']['x'], ninja['position']['y']), "negotiable": True, "buffable": True, "stacks": 0,  "value": ShinobiIdValueLookup[ninja['IdLastSave']], "range": currNinjaRange, "fullData": ninja})

allNinjas = Jitsus + Shinobis

pairs = []
for i in range(len(allNinjas)):
    for j in range(len(allNinjas)):
        if i != j:
            jitsu1 = allNinjas[i]        
            jitsu2 = allNinjas[j]
            if inRange(jitsu2, jitsu1, jitsu1['range']):
                pairs.append((i, j))

                
G = nx.Graph()
G.add_edges_from(pairs)

# nx.draw(G)

cut_value, partition = maxcut.one_exchange(G)

print(cut_value, partition)

# {2, 3, 4, 5, 6, 7, 8, 15, 16, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 35, 42, 44, 45, 50, 51, 53, 57, 58, 59, 60, 61, 66, 67, 68, 69, 71, 74, 75, 76, 83, 88, 89, 90, 91, 92, 93, 94, 99, 100, 101, 102, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 120, 129, 133, 134, 135, 138, 139}, 
# {0, 128, 130, 131, 1, 132, 136, 137, 9, 10, 12, 13, 11, 14, 17, 18, 19, 20, 21, 33, 34, 36, 37, 38, 39, 40, 41, 43, 46, 47, 48, 49, 52, 54, 55, 56, 62, 63, 64, 65, 70, 72, 73, 77, 78, 79, 80, 81, 82, 84, 85, 86, 87, 95, 96, 97, 98, 103, 104, 105, 106, 107, 108, 119, 121, 122, 123, 124, 125, 126, 127}
