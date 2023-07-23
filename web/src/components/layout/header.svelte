<script>
	import Head from '../icons/head.svelte';
    import { 
        MoonIcon, 
        BellIcon, 
        PlusCircleIcon 
    } from 'svelte-feather-icons'

</script>

<header class="navbar bg-base-100">
    <div class="flex-1 gap-6">
        <div class="w-10">
            <Head/>
        </div>
        <a class="normal-case font-extrabold text-xl" href="/"> Nine Yards </a>
    
        <input type="text" placeholder="Search" class="input search-bar w-auto hidden sm:block shadow-md" />
    </div>
    <div class="flex-none gap-2">
        <MoonIcon/>
        <PlusCircleIcon/>
        <BellIcon/>
        <div class="dropdown dropdown-end">
            <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
            <!-- svelte-ignore a11y-label-has-associated-control -->
            <label tabindex="0" class="btn btn-ghost btn-circle avatar">
                <div class="w-10 rounded-full border-l">
                    <img src="https://avatars.githubusercontent.com/u/79579164?v=4" alt="Avatar" />
                </div>
            </label>
            <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
            <ul tabindex="0" class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52">
                <li>
                    <span class="justify-between">
                        Profile
                        <span class="badge">New</span>
                    </span>
                </li>
                <li>
                    <span>Settings</span>
                </li>
                <li>
                    <span>Logout</span>
                </li>
            </ul>
        </div>
        <span class="hidden sm:block">
            Rabbitminers
        </span>
    </div>
</header>

<style scoped lang="css">

/*
Overwrite the background colour of the search bar
*/
.input.search-bar {
    @apply bg-neutral;
}

</style>