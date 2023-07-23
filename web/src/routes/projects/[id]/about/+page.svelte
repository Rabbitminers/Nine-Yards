<script lang="ts">
import Container from "../../../../components/layout/container.svelte";
import Task from "../../../../components/task/task.svelte";
	import Avatar from "../../../../components/user/avatar.svelte";
import type { Task as TaskData } from "../../../../types/tasks";

const data: TaskData = {
	id: "123",
	project_id: "456",
	task_group_id: "789",
	name: "This is a task!",
	creator: "Rabbitminers",
    position: 1,
    due: 1689807600000 + (86400 * 1000 * 3),
	primary_colour: "#ffffff",
	accent_colour: "#ffffff",
	created: 1689807600000 - (86400 * 1000 * 3)
} 

</script>

<main class="flex flex-row">
<Container title="About">
    <section class="grid grid-cols-2 gap">
        <Task {data} viewType='grid'/>
    </section>
</Container>

<Container title="Recent Updates">

</Container>
</main>
