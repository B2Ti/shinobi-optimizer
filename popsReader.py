import json
import math
import copy
import time
import sys

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

randomInit = False
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
            if randomInit or jitsuInit:
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
            if randomInit or not jitsuInit:
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




import random as rand

if randomInit:
    Shinobis3 = []
    Jitsus3 = []
    for x in range(len(Shinobis)-1):
        if rand.randint(0, 1) == 0:
            Shinobis3.append(Shinobis[x+1])
        else:
            Jitsus3.append(Jitsus[x])

    Jitsus = Jitsus3
    Shinobis = Shinobis3

allNinjas = Jitsus + Shinobis

idToNinjasInRangeLookup = {}

for jitsu in allNinjas + villages:
    currPos = jitsu['pos']
    idToNinjasInRangeLookup[jitsu['id']] = []
    for jitsu2 in allNinjas:
        if jitsu2 != jitsu:
            if inRange(jitsu2, jitsu, jitsu['range']):
                idToNinjasInRangeLookup[jitsu['id']].append(jitsu2)

ninjagonOld = ninjagon
ninjagon = Shinobis[0]

currPos = ninjagon['pos']
idToNinjasInRangeLookup[ninjagon['id']] = []
for jitsu2 in allNinjas:
    if inRange(jitsu2, ninjagon, ninjagon['range']):
        idToNinjasInRangeLookup[ninjagon['id']].append(jitsu2)

def getAttkSpdBuff(sCount):
    return 1.08**sCount
def getPierceBuff(sCount):
    return 1.08*sCount

def getSetupDps(jitsus, shinobis):
    jitsus2 = copy.deepcopy(jitsus)
    shinobis2 = copy.deepcopy(shinobis)

    setupDps = 0

    for shinobi in shinobis2:
        jitsusInRange = idToNinjasInRangeLookup[shinobi['id']]
        for jitsu in jitsusInRange:
            for jitsu2 in jitsus2: # + shinobis2
                if jitsu['id'] == jitsu2['id']:
                    jitsu2['stacks'] = min(jitsu2['stacks']+1, 20)
        if shinobi['buffable']:
            shinobi['stacks'] = min(shinobi['stacks']+1, 20)
    
    for ninja in shinobis2:
        valueBuff = getAttkSpdBuff(ninja['stacks'])
        pierceBuff = getPierceBuff(ninja['stacks'])
        ninja['value'] *= valueBuff * pierceBuff
    for ninja in jitsus2:
        valueBuff = getAttkSpdBuff(ninja['stacks'])
        pierceBuff = getPierceBuff(ninja['stacks'])
        ninja['value'] *= valueBuff * pierceBuff
    
    for ninja in shinobis2 + jitsus2:
        if ninja['negotiable']:
            setupDps += ninja['value']


    return [jitsus2, shinobis2], round(setupDps, 4)

optimised = False

def optimise(jitsus, shinobis):
    global optimised
    bestImprovement = [0, [jitsus, shinobis]]
    if not optimised:
        startTime = time.perf_counter()
        baseResults = getSetupDps(jitsus, shinobis)
        swapsResults = {}
        for jitsu in jitsus:
            if jitsu['negotiable']:
                shinobis2 = copy.deepcopy(shinobis)
                jitsus2 = copy.deepcopy(jitsus)
                newJitsu = copy.deepcopy(jitsu)
                newJitsu['value'] = ShinobiIdValueLookup[newJitsu['id']] 
                shinobis2.append(newJitsu)
                jitsus2.pop(jitsus2.index(jitsu))
                result = getSetupDps(jitsus2, shinobis2)
                swapsResults[jitsu['id']] = [round(result[1]-baseResults[1], 4), [jitsus2, shinobis2]]
        
        for shinobi in shinobis:
            if shinobi['negotiable']:
                shinobis2 = copy.deepcopy(shinobis)
                jitsus2 = copy.deepcopy(jitsus)
                newShinobi = copy.deepcopy(shinobi)
                newShinobi['value'] = JitsuIdValueLookup[newShinobi['id']] 
                jitsus2.append(newShinobi)
                shinobis2.pop(shinobis2.index(shinobi))
                result = getSetupDps(jitsus2, shinobis2)
                swapsResults[shinobi['id']] = [round(result[1]-baseResults[1], 4), [jitsus2, shinobis2]]
        
        for id in swapsResults:
            if swapsResults[id][0] > bestImprovement[0]:
                bestImprovement = swapsResults[id]
        if bestImprovement[0] == 0:
            print("No further optimisations can be made!")
            optimised = True
        endTime = time.perf_counter()
        timeTaken = endTime-startTime
        print(f"Completed one optimisation session in {timeTaken} seconds. Improved dps by {bestImprovement[0]}")
    return bestImprovement

    x=0
    



import pygame as pg
pg.font.init()
font = pg.font.SysFont('Arial', 10)

pg.init()
WHITE = ( 255, 255, 255)

gameSize = (300, 232)
size = (900, 696)

def gamePosToWindowPos(pos):
    
    windowPos = ((pos[0]*3)+(size[0]/2), (pos[1]*3)+(size[1]/2))
    return windowPos


screen = pg.display.set_mode(size)
pg.display.set_caption("Ninja stuff") 

carryOn = True
 
clock = pg.time.Clock()

Jitsus2 = copy.deepcopy(Jitsus)
Shinobis2 = copy.deepcopy(Shinobis)

def idToObj(id):
    for jitsu in Jitsus2:
        if jitsu['id'] == id:
            return jitsu
    for shinobi in Shinobis2:
        if shinobi['id'] == id:
            return shinobi
    for village in villages:
        if village['id'] == id:
            return village

def isNinjaJitsu(obj):
    return obj in Jitsus2

def isObjVillage(obj):
    return obj in villages
       

ninjaPygamePosToId = {}

for ninja in allNinjas + villages:
    ninjaPygamePosToId[gamePosToWindowPos(ninja['pos'])] = ninja['id']

ninjaSelected = None

dps = getSetupDps(Jitsus2, Shinobis2)

allNinjas2 = copy.deepcopy(allNinjas)

while carryOn:
    for event in pg.event.get(): #
        if event.type == pg.QUIT: 
            carryOn = False 
        if event.type == pg.MOUSEBUTTONDOWN:
            mousePos = event.pos
            for pos in ninjaPygamePosToId:
                obj = idToObj(ninjaPygamePosToId[pos])
                if isObjVillage(obj):
                    radius = 33
                else:
                    radius = 18
                if inRange({"pos": mousePos}, {"pos": gamePosToWindowPos(obj['pos'])}, radius):
                    ninjaSelected = obj
                    for x in dps[0][0] + dps[0][1]:
                        if x['id'] == ninjaSelected['id']:
                            print(f"stacks: {x['stacks']}, value (w shinobis): {x['value']}, value (w/o shinobis): {ninjaSelected['value']}")
                    

                    break

    improvement = optimise(Jitsus2, Shinobis2)

    allNinjas2 = improvement[1][0] + improvement[1][1]
    Jitsus2, Shinobis2 = improvement[1]

    screen.fill(WHITE)

    dps = getSetupDps(Jitsus2, Shinobis2)

    text_surface = font.render(f"Setup DPS: {dps[1]}. Ratio: {len(dps[0][0])/len(dps[0][1])}", True, (0, 0, 0))

    
    ninjasInRange = []
    if ninjaSelected:
        ninjasInRange = idToNinjasInRangeLookup[ninjaSelected['id']]
    
    for ninja in allNinjas2 + villages:
        colour = (0, 0, 0)
        selected = ninja==ninjaSelected
        if isNinjaJitsu(ninja):
            colour = (200, 200, 0)
            if ninja in ninjasInRange:
                colour = (180, 180, 0)
            if selected:
                colour = (255, 255, 0)
                
        else:
            if isObjVillage(ninja):
                colour = (0, 200, 200)
                if selected:
                    colour = (0, 255, 255)
            else:
                colour = (0, 200, 0)
                if ninja in ninjasInRange:
                    colour = (0, 180, 0)
                if selected:
                    colour = (0, 255, 0)
        
        if isObjVillage(ninja):
            pg.draw.circle(screen, colour, gamePosToWindowPos(ninja['pos']), 33)
        else:
            pg.draw.circle(screen, colour, gamePosToWindowPos(ninja['pos']), 18)
        pg.draw.circle(screen, (0, 0, 0), gamePosToWindowPos(ninja['pos']), 2)
    
    if ninjaSelected:
        circle_surface = pg.Surface((ninjaSelected['range'] * 2 * 3, ninjaSelected['range'] * 2 * 3), pg.SRCALPHA)
        circle_color = (0, 0, 0, 30) # Red with 50% transparency
        pg.draw.circle(circle_surface, circle_color, (ninjaSelected['range']*3, ninjaSelected['range']*3), ninjaSelected['range'] * 3)
        screen.blit(circle_surface, (gamePosToWindowPos(ninjaSelected['pos'])[0] - ninjaSelected['range']*3, gamePosToWindowPos(ninjaSelected['pos'])[1] - ninjaSelected['range']*3))


    

    screen.blit(text_surface, (0, 0))

    pg.display.flip()
     
    clock.tick(144)
 
pg.quit()