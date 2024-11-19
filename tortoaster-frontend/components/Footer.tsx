export function Footer(_props: {}) {
  return (
    <footer class="w-2/3 relative">
      <img
        alt="Tortoaster logo"
        src="/turtle-feet.svg"
        class="w-full object-cover mt-[4px]"
      />
      <div class="absolute right-0 top-0 text-stone-900 text-sm text-right p-8">
        {/*{% match user %}*/}
        {/*{% when Some with (user) %}*/}
        {/*Logged in as {% if let Some(name) = user.name %}{{name}}{% else %}anonymous{% endif %} | <a*/}
        {/*href="{{ logout_url }}">Log out</a>*/}
        {/*{% when None %}*/}
        {/*Viewing as guest | <a href="{{ login_url }}">Log in</a>*/}
        {/*{% endmatch %}<br/>*/}
        <a href="https://github.com/Tortoaster">GitHub</a> |{" "}
        <a href="mailto:rick@tortoaster.com">Email</a>
        <br />
        &copy; 2024 Rick van der Wal
      </div>
    </footer>
  );
}
