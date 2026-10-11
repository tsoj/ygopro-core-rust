//! Xyz Align
use super::prelude::*;

pub const ID: u32 = 60623203;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change Levels of 2 targets to a declared Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:HasLevel() and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::has_level(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsControler,1,nil,tp)
    (group::is_exists(d, sg, &|d, c| card::is_controler(d, c, tp), 1, NO_EX), false)
}
// s.nlvfilter = aux.NOT(Card.IsLevel)
fn nlvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    !card::is_level(d, c, &[lv])
}
// s.get_declarable_levels(g1,g2)
fn get_declarable_levels(d: &mut Duel, g1: &CardSet, g2: &CardSet) -> Vec<u64> {
    let mut opts = Vec::new();
    // for lv=1,12 do
    for lv in 1..=12u32 {
        // local ct=g1:FilterCount(s.nlvfilter,nil,lv)
        let ct = group::filter_count(d, g1, &|d, c| nlvfilter(d, c, lv), NO_EX);
        // if ct>1 or (ct>0 and g2:IsExists(s.nlvfilter,1,nil,lv)) then
        if ct > 1 || (ct > 0 && group::is_exists(d, g2, &|d, c| nlvfilter(d, c, lv), 1, NO_EX)) {
            // table.insert(opts,lv)
            opts.push(lv as u64);
        }
    }
    // return table.unpack(opts)
    opts
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g1=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil,e)
    let g1 = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.cfilter,tp,0,LOCATION_MZONE,nil,e)
    let g2 = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, a.e)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g1>0 and (#g1+#g2)>=2 end
    if a.chk == Some(0) {
        return !g1.is_empty() && g1.len() + g2.len() >= 2;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_LVRANK as u64);
    // local lv=Duel.AnnounceNumber(tp,s.get_declarable_levels(g1,g2))
    let opts = get_declarable_levels(d, &g1, &g2);
    let lv = duel::announce_number(d, a.tp, &opts).nil().0;
    // local g=(g1+g2):Match(s.nlvfilter,nil,lv)
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    group::match_(d, &mut g, &|d, c| nlvfilter(d, c, lv as u32), NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // e:SetLabel(lv)
    effect::set_label(d, a.e, &[lv as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(sg), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e):Filter(Card.IsFaceup,nil)
    let targets = duel::get_target_cards(d, a.e).nil();
    let g = group::filter(d, &targets, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, a.e);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if not tc:IsLevel(lv) then
        if !card::is_level(d, tc, &[lv as u32]) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // local races=g:GetBitwiseOr(Card.GetRace)
    let races = group::get_bitwise_or(d, &g, &|d, c| card::get_race(d, c) as i64);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsRace(races) end)
        effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_race(d, c, races))));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
