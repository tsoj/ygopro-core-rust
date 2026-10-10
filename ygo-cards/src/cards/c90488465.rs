//! The Chaos Creator
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 90488465;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special summon procedure (from hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon1)
    effect::set_condition(d, e1, Some(lua_fn(spcon1)));
    // e1:SetTarget(s.sptg1)
    effect::set_target(d, e1, Some(lua_fn(sptg1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(lua_fn(spop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon 1 banished monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon2)
    effect::set_condition(d, e2, Some(condition(spcon2)));
    // e2:SetTarget(s.sptg2)
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    // e2:SetOperation(s.spop2)
    effect::set_operation(d, e2, Some(operation(spop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.atchk1,1,nil,sg)
    let (ok, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, c);
    (ok && group::is_exists(d, sg, &|d, x| atchk1(d, x, sg), 1, NO_EX), false)
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and sg:FilterCount(Card.IsAttribute,c,ATTRIBUTE_DARK)==1
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && group::filter_count(d, sg, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK), c.into()) == 1
}
/// `s.spfilter1(c,att)`
fn spfilter1(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, att) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon1(e,c)`
fn spcon1(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    let e = a.e(0);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg1=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_LIGHT)
    let rg1 = duel::get_matching_group(d, Some(&|d, c| spfilter1(d, c, ATTRIBUTE_LIGHT)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg2=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_DARK)
    let rg2 = duel::get_matching_group(d, Some(&|d, c| spfilter1(d, c, ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg=rg1:Clone()
    let mut rg = rg1.clone();
    // rg:Merge(rg2)
    group::add_card(&mut rg, &rg2);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // return ft>-2 and #rg1>0 and #rg2>0 and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0)
    ft > -2
        && !rg1.is_empty()
        && !rg2.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // local g=nil
    // local rg=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_LIGHT+ATTRIBUTE_DARK)
    let rg = duel::get_matching_group(d, Some(&|d, c| spfilter1(d, c, ATTRIBUTE_LIGHT + ATTRIBUTE_DARK)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop1(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0));
    // if not g then return end
    let Value::Group(g) = g else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup() (nothing to do in Rust)
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSummonLocation(LOCATION_HAND)
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, h, LOCATION_HAND)
}
/// `s.spchk(c,e,tp)`
fn spchk(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.check(sg,e,tp,mg)`
fn check(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:IsExists(s.spchk,1,nil,e,tp) and sg:GetClassCount(Card.GetCode)==#sg
    let ok = group::is_exists(d, sg, &|d, c| spchk(d, c, e, tp), 1, NO_EX)
        && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == sg.len();
    (ok, false)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsMonster),tp,LOCATION_REMOVED,LOCATION_REMOVED,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_monster(d, c)), tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,3,3,s.check,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&check), 0, tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.check,1,tp,HINTMSG_TARGET)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&check), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg.clone()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,sg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(sg), 2, 0, 0);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, e).unwrap_or_default();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:FilterSelect(tp,s.spchk,1,1,nil,e,tp):GetFirst()
    let picked = group::filter_select(d, &g, tp, &|d, c| spchk(d, c, e, tp), 1, 1, false, NO_EX).nil();
    let sg = group::get_first(d, &picked);
    // if sg then
    if let Some(sg) = sg {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(sg), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // g:RemoveCard(sg)
        group::remove_card(&mut g, &CardSet::from_card(sg));
    }
    // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // local g1=og:Filter(Card.IsControler,nil,tp):Filter(Card.IsLocation,nil,LOCATION_DECK)
    let og1 = group::filter(d, &og, &|d, c| card::is_controler(d, c, tp), NO_EX);
    let g1 = group::filter(d, &og1, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // local g2=og:Filter(Card.IsControler,nil,1-tp):Filter(Card.IsLocation,nil,LOCATION_DECK)
    let og2 = group::filter(d, &og, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
    let g2 = group::filter(d, &og2, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.SortDeckbottom(tp,tp,#g1)
        duel::sort_deckbottom(d, tp, tp, g1.len() as u16);
    }
    // if #g2>0 then
    if !g2.is_empty() {
        // Duel.SortDeckbottom(tp,1-tp,#g2)
        duel::sort_deckbottom(d, tp, 1 - tp, g2.len() as u16);
    }
}
