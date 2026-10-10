//! Puzzlomino, the Drop-n-Deleter
use super::prelude::*;

pub const ID: u32 = 84271823;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 monsters with different Levels
    // Link.AddProcedure(c,s.mfilter,2,nil,s.matcheck)
    link::add_procedure(d, c, Some(mat_filter(|d, c, _sc, _st, _tp| mfilter(d, c))), 2, None, Some(group_check(matcheck)), None, None);
    // Change the Level of a monster(s) Special Summoned to a zone this card points to
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.lvcon)
    effect::set_condition(d, e1, Some(condition(lvcon)));
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 2 monsters (1 from each field) with the same level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and not c:IsLevel(0)
    card::has_level(d, c) && !card::is_level(d, c, &[0])
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:GetClassCount(Card.GetLevel)==#g
    group::get_class_count(d, g, &|d, c| card::get_level(d, c) as i64) == g.len()
}
/// `s.cfilter(c,tp,lg)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, lg: &CardSet) -> bool {
    // return c:IsLevelAbove(1) and c:IsFaceup() and c:IsControler(tp) and lg:IsContains(c)
    card::is_level_above(d, c, 1) && card::is_faceup(d, c) && card::is_controler(d, c, tp) && group::is_contains(lg, c)
}
/// `s.lvcon(e,tp,eg,ep,ev,re,r,rp)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let lg = card::get_linked_group(d, effect::get_handler(d, a.e).nil());
    // return eg:IsExists(s.cfilter,1,nil,tp,lg)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp, &lg), 1, NO_EX)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local lg=e:GetHandler():GetLinkedGroup()
    let lg = card::get_linked_group(d, effect::get_handler(d, a.e).nil());
    // local g=eg:Filter(s.cfilter,nil,tp,lg)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| cfilter(d, c, tp, &lg), NO_EX);
    // local cd=e:GetChainData()
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // cd.level=Duel.AnnounceLevel(tp,1,8,#g==1 and g:GetFirst():GetLevel() or nil)
    let excluded: Vec<u32> = if g.len() == 1 {
        let tc = group::get_first(d, &g).nil();
        vec![card::get_level(d, tc)]
    } else {
        vec![]
    };
    let level = duel::announce_number_range(d, tp, Some(1), Some(8), &excluded).map(|x| x.0);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, level.map_or(Value::Nil, |x| Value::Int(x as i64)));
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,g,#g,0,cd.level)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(g), n, 0, level.unwrap_or(0) as i32);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lg=e:GetHandler():GetLinkedGroup()
    let lg = card::get_linked_group(d, c);
    // local g=eg:Filter(s.cfilter,nil,tp,lg)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| cfilter(d, x, tp, &lg), NO_EX);
    // local cd=e:GetChainData()
    // --Their levels become the declared Level
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(cd.level)
        effect::set_value(d, e1, chain::effect_data(d, a.e, None, chain::field::CARD).int());
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.desfilter(c,e)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:HasLevel() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::has_level(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetControler)==2 and sg:GetClassCount(Card.GetLevel)==1
    let res = group::get_class_count(d, sg, &|d, c| card::get_controler(d, c) as i64) == 2
        && group::get_class_count(d, sg, &|d, c| card::get_level(d, c) as i64) == 1;
    (res, false)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,0,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, e)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local dg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_DESTROY)
    let dg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(dg)
    duel::set_target_card(d, dg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetTargetCards(e)
    let dg = duel::get_target_cards(d, a.e).nil();
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
