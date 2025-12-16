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

## Asteroid Defense Scenario

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant P as Planet AI

    O->>P: Asteroid(Asteroid)
    P->>O: AsteroidAck(planet_id, destroyed: bool)
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

    O->>E: StartExplorerAi
    E->>O: StartExplorerAIResult(explorer_id)
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


    O ->> newP: IncomingExplorerRequest(explorer_id, new_mpsc_sender)
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
    O ->> newP: IncomingExplorerRequest(explorer_id, new_mpsc_sender)
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
    E ->> O: GenerateResourceResponse(Option<BasicResource>, explorer_id, timestamp)
```

##  Resource Combination(manually)
```mermaid
sequenceDiagram
    participant E as Explorer
    participant P as Planet AI
    participant O as Orchestrator

    O ->> E: CombineResourceRequest(CombineResourceRequest)
    E ->> P: CombineResourceRequest(CombineResourceRequest, explorer_id)
    P ->> E: CombineResourceResponse(Result<ComplexResource, (String, Resource1, Resource2)>)
    E ->> O: CombineResourceResponse(Result, explorer_id)
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
    P ->> E: CombineResourceResponse(Result<ComplexResource, (String, Resource1, Resource2)>)

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

