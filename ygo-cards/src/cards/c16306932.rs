//! Odd-Eyes Revolution Dragon
use super::prelude::*;

pub const ID: u32 = 16306932;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // revive limit
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_REVIVE_LIMIT)
    effect::set_code(d, e0, EFFECT_REVIVE_LIMIT);
    // e0:SetCondition(s.rvlimit)
    effect::set_condition(d, e0, Some(lua_fn(rvlimit)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // splimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.psplimit)
    effect::set_target(d, e1, Some(lua_fn(psplimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon condition
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    // e3:SetValue(s.splimit)
    effect::set_value_fn(d, e3, lua_fn(splimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // special summon rule
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e4, EFFECT_SPSUMMON_PROC);
    // e4:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetRange(LOCATION_HAND)
    effect::set_range(d, e4, LOCATION_HAND);
    // e4:SetCondition(s.hspcon)
    effect::set_condition(d, e4, Some(lua_fn(hspcon)));
    // e4:SetTarget(s.hsptg)
    effect::set_target(d, e4, Some(lua_fn(hsptg)));
    // e4:SetOperation(s.hspop)
    effect::set_operation(d, e4, Some(lua_fn(hspop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // tohand
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_HAND)
    effect::set_range(d, e5, LOCATION_HAND);
    // e5:SetCost(s.thcost)
    effect::set_cost(d, e5, Some(cost_target(thcost)));
    // e5:SetTarget(s.thtg)
    effect::set_target(d, e5, Some(cost_target(thtg)));
    // e5:SetOperation(s.thop)
    effect::set_operation(d, e5, Some(operation(thop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // atk/def
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e6, EFFECT_UPDATE_ATTACK);
    // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetValue(s.atkval)
    effect::set_value_fn(d, e6, effect_card_value(atkval));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e6:Clone()
    let e7 = effect::clone(d, e6);
    // e7:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e7, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // todeck
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e8, CATEGORY_TODECK);
    // e8:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e8, EFFECT_TYPE_IGNITION);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetCost(s.tdcost)
    effect::set_cost(d, e8, Some(cost_target(tdcost)));
    // e8:SetTarget(s.tdtg)
    effect::set_target(d, e8, Some(cost_target(tdtg)));
    // e8:SetOperation(s.tdop)
    effect::set_operation(d, e8, Some(operation(tdop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
// function s.rvlimit(e)
fn rvlimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_HAND)
    let c = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, c, LOCATION_HAND)
}
// function s.psplimit(e,c,tp,sumtp,sumpos)
fn psplimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtp = a.int(3) as u64;
    // return not c:IsRace(RACE_DRAGON) and (sumtp&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM
    !card::is_race(d, c, RACE_DRAGON) && (sumtp & SUMMON_TYPE_PENDULUM as u64) == SUMMON_TYPE_PENDULUM as u64
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsType(TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DRAGON)
        && card::is_type(d, c, TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)~=0 and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None) != 0 {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    let st = a.int(3) as u64;
    let c = effect::get_handler(d, a.e(0)).nil();
    // return (st&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM and e:GetHandler():IsLocation(LOCATION_HAND)
    (st & SUMMON_TYPE_PENDULUM as u64) == SUMMON_TYPE_PENDULUM as u64 && card::is_location(d, c, LOCATION_HAND)
}
// function s.hspfilter(c,e,tp)
fn hspfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsType(TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ)
}
// function s.hspcon(e,c)
fn hspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetReleaseGroup(tp):Filter(s.hspfilter,nil)
    let rg = duel::get_release_group(d, tp, false, false, None);
    let rg = group::filter(d, &rg, &|d, c| hspfilter(d, c), NO_EX);
    // local g1=rg:Filter(Card.IsType,nil,TYPE_FUSION)
    let g1 = group::filter(d, &rg, &|d, c| card::is_type(d, c, TYPE_FUSION), NO_EX);
    // local g2=rg:Filter(Card.IsType,nil,TYPE_SYNCHRO)
    let g2 = group::filter(d, &rg, &|d, c| card::is_type(d, c, TYPE_SYNCHRO), NO_EX);
    // local g3=rg:Filter(Card.IsType,nil,TYPE_XYZ)
    let g3 = group::filter(d, &rg, &|d, c| card::is_type(d, c, TYPE_XYZ), NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3 and #g1>0 and #g2>0 and #g3>0
    //     and aux.SelectUnselectGroup(g1,e,tp,1,1,aux.ChkfMMZ(1),0)
    //     and aux.SelectUnselectGroup(g2,e,tp,1,1,aux.ChkfMMZ(1),0)
    //     and aux.SelectUnselectGroup(g3,e,tp,1,1,aux.ChkfMMZ(1),0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -3
        && !g1.is_empty()
        && !g2.is_empty()
        && !g3.is_empty()
        && matches!(aux::select_unselect_group(d, &g1, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false), Err(true))
        && matches!(aux::select_unselect_group(d, &g2, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false), Err(true))
        && matches!(aux::select_unselect_group(d, &g3, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false), Err(true))
}
// function s.hsptg(e,tp,eg,ep,ev,re,r,rp,c)
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetReleaseGroup(tp):Filter(s.hspfilter,nil)
    let rg = duel::get_release_group(d, tp, false, false, None);
    let rg = group::filter(d, &rg, &|d, c| hspfilter(d, c), NO_EX);
    // local g1=rg:Filter(Card.IsType,nil,TYPE_FUSION)
    let g1 = group::filter(d, &rg, &|d, c| card::is_type(d, c, TYPE_FUSION), NO_EX);
    // local g2=rg:Filter(Card.IsType,nil,TYPE_SYNCHRO)
    let g2 = group::filter(d, &rg, &|d, c| card::is_type(d, c, TYPE_SYNCHRO), NO_EX);
    // local g3=rg:Filter(Card.IsType,nil,TYPE_XYZ)
    let g3 = group::filter(d, &rg, &|d, c| card::is_type(d, c, TYPE_XYZ), NO_EX);
    // local mg1=aux.SelectUnselectGroup(g1,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_RELEASE,nil,nil,true)
    let mut mg1 = aux::select_unselect_group(d, &g1, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true)
        .unwrap_or_default();
    // if #mg1>0 then
    if !mg1.is_empty() {
        // local mg2=aux.SelectUnselectGroup(g2,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_RELEASE,nil,nil,true)
        let mg2 = aux::select_unselect_group(d, &g2, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true)
            .unwrap_or_default();
        // mg1:Merge(mg2)
        group::add_card(&mut mg1, &mg2);
        // if #mg1>1 then
        if mg1.len() > 1 {
            // local mg3=aux.SelectUnselectGroup(g3,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_RELEASE,nil,nil,true)
            let mg3 = aux::select_unselect_group(d, &g3, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true)
                .unwrap_or_default();
            // mg1:Merge(mg3)
            group::add_card(&mut mg1, &mg3);
        }
    }
    // if #mg1==3 then
    if mg1.len() == 3 {
        // mg1:KeepAlive()
        // e:SetLabelObject(mg1)
        effect::set_label_object(d, e, Value::Group(mg1));
        // return true
        return true;
    }
    // return false
    false
}
// function s.hspop(e,tp,eg,ep,ev,re,r,rp,c)
fn hspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let Some(g) = effect::get_label_object(d, a.e(0)).as_group().cloned() else {
        // if not g then return end
        return;
    };
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
    // g:DeleteGroup()
}
// function s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsDiscardable() and Duel.CheckLPCost(tp,500) end
    if a.chk == Some(0) {
        return card::is_discardable(d, c, None, None) && duel::check_lp_cost(d, a.tp, 500);
    }
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, CardSet::from_card(c), REASON_COST | REASON_DISCARD, None, None);
    // Duel.PayLPCost(tp,500)
    duel::pay_lp_cost(d, a.tp, 500);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsRace(RACE_DRAGON) and c:IsLevelBelow(8) and c:IsAbleToHand()
    card::is_type(d, c, TYPE_PENDULUM) && card::is_race(d, c, RACE_DRAGON) && card::is_level_below(d, c, 8) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return math.floor(Duel.GetLP(1-e:GetHandlerPlayer())/2)
    let p = effect::get_handler_player(d, e);
    (duel::get_lp(d, 1 - p) as i64).div_euclid(2)
}
// function s.tdcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let cost = (duel::get_lp(d, a.tp) as i64).div_euclid(2);
    duel::pay_lp_cost(d, a.tp, cost as u32);
    true
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, c.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_deck(d, x)), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, c.into());
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
