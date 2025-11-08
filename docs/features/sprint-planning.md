# Sprint Planning

This document covers Jility's sprint planning features, including configurable sprint capacity and sprint rollover functionality.

## Sprint Capacity

Sprint capacity represents the target amount of work (in story points) your team can realistically complete in one sprint.

### Default Capacity

When you first use sprint planning, Jility automatically sets capacity based on your team's historical performance:

- **With sprint history**: Defaults to your team's average velocity from completed sprints
- **No sprint history**: Defaults to 40 points

### Editing Capacity

You can customize sprint capacity at any time:

1. Navigate to Sprint Planning
2. Click the capacity value (displayed as "X pts")
3. Enter your desired capacity
4. Press Enter or click the checkmark to save

The capacity setting is workspace-specific and persists across sessions.

### Capacity Indicator

The capacity indicator shows how much work is planned for the sprint:

- **Green (0-80%)**: Comfortable sprint load
- **Yellow (80-100%)**: Approaching capacity
- **Red (>100%)**: Over capacity - consider removing tickets

### Best Practices

- Review capacity at the start of each sprint
- Adjust based on team availability (holidays, vacation, etc.)
- Use historical velocity as a guide, not a hard rule
- Leave some buffer capacity for unexpected work

### Technical Implementation

**Current State**: Sprint capacity is stored in localStorage as a temporary solution. Each workspace has its own capacity setting.

**Future Enhancement**: When the backend workspace settings API is available, capacity will sync across devices and be stored in the database. See `docs/plans/backend-migration-sprint-capacity.md` for migration details.

## Sprint Rollover

When completing a sprint that has incomplete tickets, Jility provides three options:

### Roll over to next sprint (default)
- Creates a new sprint with an auto-incremented name
- Moves all incomplete tickets to the new sprint
- Copies the sprint goal from the current sprint
- New sprint starts in "planning" status

**Sprint name examples:**
- "Sprint 1" → "Sprint 2"
- "Q2 Sprint 5" → "Q2 Sprint 6"
- "Release 2.0" → "Release 3.0"

### Return to backlog
- Removes incomplete tickets from the sprint
- Changes ticket status to "backlog"
- Tickets become available for future sprint planning

### Keep in this sprint
- Leaves incomplete tickets associated with the completed sprint
- Useful for retrospectives and sprint analysis
- Incomplete tickets remain in their current status

### When all tickets are complete
If all tickets in a sprint are marked as "done", the completion dialog simply confirms the action without showing rollover options.

## Sprint Planning Workflow

### Creating a Sprint

1. Navigate to Sprint Planning
2. Click "Create Sprint" button
3. Enter a sprint name (e.g., "Sprint 1", "Q1 Sprint 3")
4. Optionally set a sprint goal
5. Click "Create"

### Adding Tickets to a Sprint

1. View the backlog tickets on the left side of the planning page
2. Drag and drop tickets into the current sprint section
3. Monitor the capacity indicator to avoid over-committing
4. Tickets added to a sprint remain in their current status (backlog, todo, etc.)

### Starting a Sprint

1. Ensure the sprint has tickets assigned
2. Click "Start Sprint" button
3. Sprint status changes from "planning" to "active"
4. Only one sprint can be active at a time

### Completing a Sprint

1. Navigate to the Active Sprint page
2. Review incomplete tickets (if any)
3. Click "Complete Sprint" button
4. Choose how to handle incomplete tickets:
   - Roll over to next sprint (creates new sprint automatically)
   - Return to backlog (removes from sprint)
   - Keep in this sprint (leaves as-is)
5. Click "Complete Sprint" in the dialog to confirm

### Viewing Sprint History

1. Navigate to Sprint History
2. View all completed sprints
3. Click on a sprint to see its details
4. Review velocity metrics and completion statistics

## Related Documentation

- **Backend Migration Plan**: See `docs/plans/backend-migration-sprint-capacity.md` for details on moving capacity from localStorage to database storage
- **Implementation Plans**:
  - Sprint Rollover: `docs/plans/2025-11-08-sprint-rollover.md`
  - Configurable Capacity: `docs/plans/2025-11-08-configurable-sprint-capacity.md`
