package {{packageName}}.web;

import {{packageName}}.service.{{entity.name}}Service;
import {{packageName}}.entity.{{entity.name}};

@RestController
@RequestMapping("/{{entity.name}}")
public class {{entity.name}}Api {
	
	@Autowired
	private {{entity.name}}Service {{entity.low_case_name}}Service;

	@RequestMapping(value = "/list", method = RequestMethod.GET)
	public List<{{entity.name}}> list() {
		return {{entity.low_case_name}}Service.list();
	}

	@RequestMapping(value = "/{id}", method = RequestMethod.GET)
	public {{entity.name}} get(@PathVariable("id") Long id) {
		return {{entity.low_case_name}}Service.get(id);
	}

	@RequestMapping(value = "/save", method = RequestMethod.POST)
	public {{entity.name}} save({{entity.name}} {{entity.low_case_name}}) {
		return {{entity.low_case_name}}Service.save({{entity.name}});
	}

	@RequestMapping(value = "/update", method = RequestMethod.POST)
	public {{entity.name}} update({{entity.name}} {{entity.low_case_name}}) {
		return {{entity.low_case_name}}Service.update({{entity.name}});
	}

	@RequestMapping(value = "/delete/{id}", method = RequestMethod.DELETE)
	public void delete(@PathVariable("id") Long id) {
		{{entity.low_case_name}}Service.delete(id);
	}
}