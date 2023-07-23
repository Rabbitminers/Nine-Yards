<script lang="ts">
	import type { Task } from '../../types/tasks';

    import { daysUntil, formatDate } from '../../utils/date'
    import { MoreVerticalIcon, } from 'svelte-feather-icons'

    export let data: Task;

    const progress = 10; // TODO: make dynamic
    const task_group_name = "Doing"; // TODO: make dynamic

    const getRemaining = (): string => {
        if (data.due === undefined) {
            return "No Due Date";
        }
        const remaining: number = daysUntil(data.due);
        return remaining >= 0 ? `${remaining} Days Left` : `${Math.abs(remaining)} Days ago` 
    }

    const refreshData = async () => {
        // TODO
    }
</script>

<article class="card bg-base-100 w-full min-w-max">
    <section class="card-body items-center text-center">
        <section class="flex flex-row w-full justify-between">
            <span>{formatDate(data.created)}</span>
            <MoreVerticalIcon/>
        </section>
        <h2 class="card-title">{data.name}</h2>
        <p>{task_group_name}</p>
        <span class="text-left w-full text-sm font-extrabold">Progress</span>
        <progress class="progress w-56" value={progress} max="100"></progress>
        <span class="text-right w-full text-sm font-extrabold">{progress}%</span>
        <section class="flex w-full items-center justify-between">
            <section class="avatar-group -space-x-3">
                <div class="avatar">
                    <div class="w-6">
                        <img src="https://avatars.githubusercontent.com/u/79579164?v=4" alt="avatar" />
                    </div>
                </div>
                <div class="avatar placeholder">
                    <div class="w-6 bg-neutral-focus text-neutral-content text-xs">
                        <span>+2</span>
                    </div>
                </div>
            </section>
            <div class="badge badge-neutral text-xs"> {getRemaining()} </div>
        </section>
    </section>
</article>