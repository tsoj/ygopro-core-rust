//! Cyberdark Impact!
use super::prelude::*;

pub const ID: u32 = 80033124;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[40418351, 41230939, 77625948, 3019642], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsCode,40418351),Card.IsAbleToDeck,s.fextra,Fusion.ShuffleMaterial,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_code(d, c, &[40418351]))),
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::shuffle_material()),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.frec(c,tp,sg,g,code,...)`: whether `c` can be the next material with the codes left in `codes`
/// (`g` holds the materials chosen before it)
fn frec(d: &mut Duel, c: CardId, tp: u8, sg: &CardSet, g: &CardSet, codes: &[u32]) -> bool {
    // if not c:IsSummonCode(fc,SUMMON_TYPE_FUSION,tp,code) then return false end
    let code = codes[0];
    if !card::is_summon_code(d, c, None, SUMMON_TYPE_FUSION as u64, tp, &[code]) {
        return false;
    }
    // if ... then
    if codes.len() > 1 {
        // g:AddCard(c)
        let mut g2 = g.clone();
        group::add_card(&mut g2, &CardSet::from_card(c));
        // local res=sg:IsExists(s.frec,1,g,tp,sg,g,...)
        let res = group::is_exists(d, sg, &|d, c2| frec(d, c2, tp, sg, &g2, &codes[1..]), 1, (&g2).into());
        // g:RemoveCard(c)
        // return res
        res
    } else {
        // else return true end
        true
    }
}
/// `s.fcheck(tp,sg,fc,mg)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(0);
    let sg = a.g(1);
    // return #sg==3 and sg:IsExists(s.frec,1,nil,tp,sg,Group.CreateGroup(),41230939,77625948,3019642)
    sg.len() == 3 && group::is_exists(d, &sg, &|d, c| frec(d, c, tp, &sg, &CardSet::new(), &[41230939, 77625948, 3019642]), 1, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(Card.IsAbleToDeck),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| card::is_able_to_deck(d, c))), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
    // return g,s.fcheck
    (Some(g), Some(lua_fn(fcheck)))
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
