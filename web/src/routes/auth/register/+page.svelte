<script lang="ts">
    import { UserIcon, MailIcon, LockIcon } from 'svelte-feather-icons';
</script>

<article class="card-body">
    <span class="card-title pb-3">
        Welcome!
    </span>
    <section class="flex flex-col gap-3">
        <div class="form-control">
            <label class="label">
                <span class="label-text flex gap-2 items-center"> 
                    <UserIcon/> 
                    Username
                </span>
            </label>
            <input type="text" placeholder="username" class="input input-bordered" />
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text flex gap-2 items-center"> 
                    <MailIcon/> 
                    Email
                </span>
            </label>
            <input type="text" placeholder="email" class="input input-bordered" />
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text flex gap-2 items-center"> 
                    <LockIcon/> 
                    Password
                </span>
            </label>

            <input type="password" placeholder="password" class="input input-bordered" />

            <label class="label">
                <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
            </label>

            <button class="btn button-wide"> 
                Register
            </button>
        </div>

        <a href="login">Already have an account?</a>
    </section>
</article>