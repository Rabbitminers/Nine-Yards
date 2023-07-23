<script lang="ts">
    import { MailIcon, LockIcon } from 'svelte-feather-icons';
</script>

<article class="card-body">
    <span class="card-title pb-3">
        Welcome Back!
    </span>
    <section class="flex flex-col gap-3">
        <div class="form-control">
            <label class="label">
                <span class="label-text flex gap-2 items-center"> 
                    <MailIcon/> 
                    Username or Email
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
                Login
            </button>
        </div>

        <a href="register">Dont Have An Account?</a>
    </section>
</article>