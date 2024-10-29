# Bevy Concepts

## Application

The Application is the main entry point of Bevy. It manages the game loop, schedules systems, and handles resources and events. It exists only at build time.

## Plugin

A Plugin is a modular piece of functionality that can be added to a Bevy app. It encapsulates systems, resources, and configuration. It exists only at build time.

## World

The World is a data structure that stores all entities and their components. It provides methods to query and manipulate entities and components.

## Query

A Query is used to access entities and their components in a World. It allows systems to filter and iterate over entities with specific component sets.

## Commands

Commands are used to schedule changes to the World, such as adding or removing entities and components. They are executed at synchronization points.

## Resource

A Resource is a globally accessible data structure that is not tied to any specific entity. It is used to store shared data and state.

## Event

An Event is a message that can be sent and received by systems. Events are used to communicate between systems and decouple their logic.

## Observer

An Observer is a system that reacts to changes in the World, such as component modifications or entity creation. It is used to implement reactive behavior.

## Tag Component

A Tag Component is a zero-sized type used to mark entities with a specific characteristic or role. It is used for efficient filtering and querying.
