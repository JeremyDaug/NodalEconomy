# Nodal Economy

This is an attempt to create a economic simulator that is 'headless' with no central market, instead all information is distributed across the various participants in the market. If possible, using this for some kind of game, or at least some kind of economic simulator toy.

# System Breakdown

## Factuals

Factuals can be thought of as the 'truths' of the system being run. This includes (base) goods, processes, and the underlying rules that are otherwise divorced from the 'material' reality of the rest of the system.

### Goods
Goods are specific things that are desireable, either directly or indirectly. These can include vague things things like Calories and Nutrition, abstract things like emotions or belonging, generic goods such as 'bread' or 'water'. These are independent (or nearly independent) goods. 

For more specific varieties of goods (like brands of bread), or modified goods, like preserved goods, bulk goods, packaged goods, disassembled goods and so on, that would be a Package.

### Processes
Processes take in packages of goods and transform them into other packages. The simplest form takes in goods and outputs other types of goods, but processes can also include adding or removing Parts of a package, grouping or ungrouping multiple goods into a singular package, and many other things. 

Some processes are tied to Land, Resource Nodes, and other thing, but in general represent any activity that can be done by intelligent beings. Some processes will be used by wildlife to represent their growth and needs, but they will be much more limited in their abilities.

## Environment
The Environment is everything that exists and is considered static. This includes land, resources, and locations. This can include both Flora and Fauna if they are downgraded from Wildlife, at the cost of them being much more finite and static.

### Resource Nodes
Resource Nodes are resource generators. These can be finite or infinite, and can be claimed. Claiming them requires additional work though. They also typically have processes attached to them that define how they are worked.

### Land
Land is generic space, needs to be used in most processes to do work. Land has climactic and soil value, representing it's efficiency in various uses. 

Currently, this is simplified into simple terrain types.
- Desert
- Wasteland
- Plains
- Grassland
- Black Earth
- Floodplain
- Hills
- Mountains
- Rivers
- Lakes
Desert, Wasteland, Plains, Grassland, Black Earth, and Floodplain are all flat and open by default and have fertility that ranges from least to most fertile. 

Hills and Mountains are rough terrain and require either special work to make useable, or specific processes to use them directly.

Rivers and Lakes cannot be used like normal land, act as water sources, are rough terrain, but can be traversed more rapidly by boats and other water transports.

### Locales
Locales are collections of land, resources, and actors. Whatever happens to be the lowest level of locale is where people are capable of acting and interacting casually. Anything outside that lowest level requires additional time in the form of travel.

The current layout for version 1 is as follows.
- Locale
- Province
- State
- Region
- Continent
- World

### Sea
Sea is territory of unconstructable space. It requires special transports or extra effort to cross. These are considered locales, and so can be combined into sea provinces, states, etc. Ideally, they also include information like currents and weather, and may eventually be seasteaded.

### Buildings
Buildings are a method to create artificial land and consolidate resource storage into one separate location. Actors, Groups, Jobs, and especially Corporations use them as storage for goods in the long term as well as a common location for information to be stored.

## Participants
Participants are the 'active' parts of the system.

### Wildlife
Wildlife is any non-human level creature. This can include both plants and animals. They do stuff, consume resources, and produce outputs, but not much else directly. If evolution is allowed to exist, then they periodically change and alter to try and match the environment more appropriately.

Wildlife has a highly limited memory, property, and possible actions. Most of those actions are purely centered on survival.

### Actors
Actors are the smallest unit of economic actors. They produce time each day based on their population, and use that time to try and satisfy it's desires. Each actor is fully independent of each other, but those within an actor act in lockstep, multiplying all effects to meet it's population needs.

Pops have a limited Memory, Relationship, and Property. They also have access to almost any action they, though they must either know about the action themselves or be able to work someplace that does.

They also have a size and household breakdown, which defines aletrations to consumption and working time. And their needs are defined by desires, which can also alter over time.

### Groups
Groups are collections of actors working together, like a village, commune, community, or similar type of organization. Hypothetically, they can be used to consolidate and collect resources for a wider group of actors, reducing computational demand, but we'll have to see if that's possible, necissary, or feaseable.

### Firm
Firm are a more complex actor. Capable of gathering and doing multiple things. The biggest thing it's capable of is organizing production chains. This reduces friction between work, and it can also act as a merchant, buying goods to resell them. Firms are expressly owned by a singular actor who controls them directly. It can also act as a kind of shared institution which maintains informatino across the market.

The actor of a firm may spend a portion of their time managing the firm instead of working the jobs in the firm.

### Corporations
A Corporation is a more advanced firm, instead of being run by a singular Actor it's run by a group of shareholders and stakeholders who get a vote in how the corporation is run. They may also have much wider and complex structures then Firms.

### Institution
Institutions are non-economic organizations which lack the force of aggression. They offer broader services, store cultural information, and do similar activities.

### State
States are groups which organize and have access to aggression to enforce their will. They deal with complex factors like legitimacy and the like. In many ways they act like a crystallization of institutions, firms, and corporations into a great power.

## Packages and Parts
Packages are groups of data, typically centered on a good, but it can also be used for other types of data or modifiers on a good, such as the state of the good, it's preservation, and so on.

Existing Parts of a package are listed below.
- Subpack (Package) - Contains a package within this package allowing for nested packages. Sub-packages have some rules to restrict them, mostly related to stacking stuff like storage or otherwise physically impossble info.
- Good (id, quantity) - A good and how many units of that good we 
- Quality
- Preserved
- Voucher
- Disassembled
- Record

## Species
A species is the physical aspect of all participants. Wildlife is defined almost exclusively by them, while Actors and above typically have this as their baseline, while the rest of their drive comes from desires, 

## Memories
Memories are pieces of data stored in varios locations and represent useful data to the actors.
This includes Skills, Prices, Relationships, and so on.

## Household
A breakdown of members of an actor. Actors are made of households rather than individuals as it makes continuity between individuals easy.

## Desires
Desires are what is being sought out. The species of a participant defines what they need for basic life, while these are what they need for additional satisfaction. It's typically limited to Actors, but some groups may also have 'collective' desires, representing a kind of shared organizational need.