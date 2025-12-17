# Message Diagrams
This file provides a series of example conversations to be used as a reference when implementing the common crate.


## Planet Initialization

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant P as Planet AI

    O->>P: StartPlanetAI
    P->>O: StartPlanetAIResult(planet_id)
```

## Planet AI Stop

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant P as Planet AI

    O->>P: StopPlanetAI
    P->>O: StopPlanetAIResult(planet_id)
```

## Sunray Interaction

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant P as Planet AI

    O->>P: Sunray(Sunray)
    P->>O: SunrayAck(planet_id)
```

## Planet is Killed
this must always be handled, even if Planet is Stopped

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant P as Planet AI
    
    O->>P: KillPlanet
    P->>O: KillPlanetResult(planet_id)
```

## Asteroid Defense Scenario

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant P as Planet AI
    
    O->>P: Asteroid(Asteroid)
    alt Planet has Rocket
    P->>O: AsteroidAck(planet_id, Some(Rocket))
    else Planet does NOT have a Rocket
    P->>O: AsteroidAck(planet_id,None)
    O->>P: KillPlanet
    P->>O: KillPlanetResult(planet_id)
    end
```

## Internal State Discovery

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant P as Planet

    O->>P: InternalStateRequest
    P->>O: InternalStateResponse(planet_id, DummyPlanetState)
```

## Explorer Initialization

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant E as Explorer

    O->>E: StartExplorerAI
    E->>O: StartExplorerAIResult(explorer_id)
```

## Reset Explorer

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant E as Explorer

    O->>E: ResetExplorerAI
    E->>O: ResetExplorerAIResult(explorer_id)
```
## Stop Explorer

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant E as Explorer

    O->>E: StopExplorerAI
    E->>O: StopExplorerAIResult(explorer_id)
```

## Neighbors discovery

```mermaid
sequenceDiagram
    participant E as Explorer
    participant O as Orchestrator

    E->>O: NeighborsRequest(explorer_id, current_planet_id)
    O->>E: NeighborsResponse(Vec<planet_id>)
```

## Moving to another planet (Manually from Orch)

```mermaid
sequenceDiagram
    participant E as Explorer
    participant O as Orchestrator
    participant newP as New Planet
    participant currP as Current Planet


    O ->> newP: IncomingExplorerRequest(explorer_id, new_sender)
    newP ->> O: IncomingExplorerResponse(planet_id, Result)
    O ->> currP: OutgoingExplorerRequest(explorer_id)
    currP ->> O: OutgoingExplorerRequest(planet_id, Result)

    O ->>E: MoveToPlanet(channelof_new_planet)
    E->> O: MovedToPlanetResult(explorer_id)
```

## Moving to another planet (Explorer Asks)

```mermaid
sequenceDiagram
    participant E as Explorer
    participant O as Orchestrator
    participant newP as New Planet
    participant currP as Current Planet



    E->>O: TravelToPlanet(explorer_id, start_planet_id, dst_planet_id)
    O ->> newP: IncomingExplorerRequest(explorer_id, new_sender)
    newP ->> O: IncomingExplorerResponse(planet_id, Result)
    O ->> currP: OutgoingExplorerRequest(explorer_id)
    currP ->> O: OutgoingExplorerRequest(planet_id, Result)

    O ->>E: MoveToPlanet(channel_of_new_planet)
    E->> O: MovedToPlanetResult(explorer_id)
```
## Bag Content

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant E as Explorer

    O->>E: BagContentRequest
    E->>O: BagContentResponse(explorer_id, ExplorerBag)
```

## Basic Resource discovery(manually)
```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI
    participant O as Orchestrator

    O ->> E: SupportedResourceRequest
    E ->> P: SupportedResourceRequest(explorer_id)
    P ->> E: SupportedResourceResponse(resource_list)
    E ->> O: SupportedResourceResponse(resource_list, explorer_id)
```
## Combination Rules discovery(manually)
```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI
    participant O as Orchestrator

    O ->> E: SupportedCombinationRequest
    E ->> P: SupportedCombinationRequest(explorer_id)
    P ->> E: SupportedCombinationResponse(comb_list)
    E ->> O: SupportedCombinationResponse(comb_list, explorer_id)
```
## Basic Resource Generation(manually)
```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI
    participant O as Orchestrator

    O ->> E: GenerateResourceRequest(res_to_generate)
    E ->> P: GenerateResourceRequest(explorer_id, res_to_generate)
    P ->> E: GenerateResourceResponse(Option<BasicResource>)
    alt Resource is generated
    E ->> O: GenerateResourceResponse(Ok(), explorer_id)
    else Resource is not generated
    E ->> O: GenerateResourceResponse(Err(String), explorer_id)
    end
```

##  Resource Combination(manually)
```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI
    participant O as Orchestrator

    O ->> E: CombineResourceRequest(CombineResourceRequest)
    E ->> P: CombineResourceRequest(CombineResourceRequest, explorer_id)
    alt Resource is generated
    E ->> O: GenerateResourceResponse(Ok(), explorer_id)
    else Resource is not generated
    E ->> O: GenerateResourceResponse(Err(String), explorer_id)
    end
```

## Basic Resource discovery (from Explorer)

```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI

    E ->> P: SupportedResourceRequest(explorer_id)
    P ->> E: SupportedResourceResponse(resource_list)
```

## Combination Rules discovery(from Explorer)
```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI

    E ->> P: SupportedCombinationRequest(explorer_id)
    P ->> E: SupportedCombinationResponse(comb_list)

```

## Basic Resource Generation (from Explorer)
```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI

    E ->> P: GenerateResourceRequest(explorer_id, res_to_generate)
    P ->> E: GenerateResourceResponse(Option<BasicResource>)
```


##  Resource Combination(from Explorer)
```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI

    E ->> P: CombineResourceRequest(CombineResourceRequest, explorer_id)
    alt Complex Resource is generated
    P ->> E: CombineResourceResponse(Ok(ComplexResource))
    else Complex Resource is not generated
    P ->> E: CombineResourceResponse(Err((String, Resource1, Resource2)))
    end


```

## Energy Cell Availability

```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI

    E->>P: AvailableEnergyCellRequest(explorer_id)
    P->>E: AvailableEnergyCellResponse(available_cells_qty)
```


## Internal State Discovery (from Explorer)

```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet

    E->>P: InternalStateRequest(explorer_id)
    P->>E: InternalStateResponse(PlanetState)
```

