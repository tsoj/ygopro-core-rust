//! Archidux, the Squid Sorcerer
//! JP name
use super::prelude::*;

pub const ID: u32 = 101402086;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))), 2, None, None, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.atkdefcost)
    effect::set_cost(d, e1, Some(cost_target(atkdefcost)));
    // e1:SetOperation(s.atkdefop)
    effect::set_operation(d, e1, Some(operation(atkdefop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local g=Group.CreateGroup()
    // (the group is the label object of e2 and changed by e3: e3 reaches it through e2)
    let g = CardSet::new();
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // e2:SetLabelObject(g)
    effect::set_label_object(d, e2, Value::Group(g));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end) (g is e2's label object: see below)
    effect::set_operation(d, e3, Some(operation(destroyed_op)));
    // the function uses the upvalue g: its label object is e2, which holds the group
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.atkdefcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkdefcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// s.atkdefop(e,tp,eg,ep,ev,re,r,rp)
fn atkdefop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1a, LOCATION_MZONE, 0);
    // e1a:SetTarget(function(e,c) return c:IsAttribute(ATTRIBUTE_WATER) end)
    effect::set_target(d, e1a, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_WATER))));
    // e1a:SetValue(500)
    effect::set_value(d, e1a, 500);
    // e1a:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1a, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1a,tp)
    duel::register_effect(d, e1a, tp);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // Duel.RegisterEffect(e1b,tp)
    duel::register_effect(d, e1b, tp);
}
// s.tgfilter(c,e,tp,opp,ign_chk)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, opp: u8, ign_chk: bool) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp) and c:IsPreviousAttributeOnField(ATTRIBUTE_WATER)
    //     and c:IsLinkMonster() and c:IsFaceup() and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and c:IsCanBeEffectTarget(e)
    //     and c:IsReason(REASON_EFFECT) and c:IsReasonPlayer(opp)
    //     and (ign_chk or Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp,c:GetLink()))
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_attribute_on_field(d, c, ATTRIBUTE_WATER)
        && card::is_link_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_reason_player(d, c, opp)
        && (ign_chk || {
            let link = card::get_link(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, link)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
        })
}
// s.spfilter(c,e,tp,link)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, link: u32) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsLevelBelow(link) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_level_below(d, c, link) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let opp = 1 - tp;
    // local g=e:GetLabelObject():Filter(s.tgfilter,nil,e,tp,opp)
    let mut g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // (Filter is evaluated here: the card's group, filtered)
    let e = a.e;
    g = group::filter(d, &g, &|d, c| tgfilter(d, c, e, tp, opp, false), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.tgfilter(chkc,e,tp,opp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && tgfilter(d, chkc, e, tp, opp, false);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && !g.is_empty();
    }
    // local tc=nil
    // if #g>1 then Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET); tc=g:Select(tp,1,1,nil):GetFirst()
    // else tc=g:GetFirst() end
    let tc = if g.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        group::get_first(d, &g)
    };
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp,tc:GetLink())
        let link = card::get_link(d, tc);
        let e = a.e;
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(move |d, x| spfilter(d, x, e, tp, link))),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
// e3's operation: function(e,tp,eg,ep,ev,re,r,rp) ... end (g: the group of e2, reached through e3's label object)
fn destroyed_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.IsDamageStep() or eg:IsContains(c) then return end
    if duel::is_damage_step(d, None) || group::is_contains(&a.eg.clone().nil(), c) {
        return;
    }
    // local tg=eg:Filter(s.tgfilter,nil,e,tp,1-tp,true)
    let eg = a.eg.clone().nil();
    let opp = 1 - tp;
    let e2 = match effect::get_label_object(d, a.e) {
        Value::Effect(e2) => e2,
        _ => return,
    };
    let e = a.e;
    let tg = group::filter(d, &eg, &|d, x| tgfilter(d, x, e, tp, opp, true), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g = the group of e2 (its label object)
        let mut g = effect::get_label_object(d, e2).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return not c:HasFlagEffect(id) end,nil)
        group::remove(d, &mut g, &|d, x| !card::has_flag_effect(d, x, ID, None), NO_EX);
        // (store the group back into the label object of e2)
        effect::set_label_object(d, e2, Value::Group(g));
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, tp, tp, 0);
    }
}
