//! Miscellaneousaurus
use super::prelude::*;

pub const ID: u32 = 38572779;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.immcon)
    effect::set_condition(d, e1, Some(condition(immcon)));
    // e1:SetCost(s.immcost)
    effect::set_cost(d, e1, Some(cost_target(immcost)));
    // e1:SetOperation(s.immop)
    effect::set_operation(d, e1, Some(operation(immop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.immcon(e,tp,eg,ep,ev,re,r,rp)`
fn immcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.immcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn immcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() and Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_grave_as_cost(d, h) && duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_COST, None, None);
    true
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetOwnerPlayer()~=re:GetOwnerPlayer() and re:IsActivated()
    let e = a.e(0);
    let re = a.e(1);
    effect::get_owner_player(d, e) != effect::get_owner_player(d, re) && effect::is_activated(d, re)
}
/// `s.immop(e,tp,eg,ep,ev,re,r,rp)`
fn immop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_DINOSAUR))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_DINOSAUR))));
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // if Duel.IsPhase(PHASE_MAIN1) then
    if duel::is_phase(d, PHASE_MAIN1) {
        // e1:SetReset(RESET_PHASE|PHASE_MAIN1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_MAIN1, 1);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_MAIN2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_MAIN2, 1);
    }
    // if Duel.IsPhase(PHASE_MAIN1) then
    if duel::is_phase(d, PHASE_MAIN1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_MAIN1,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_MAIN1, 0, 1, 0);
    } else {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_MAIN2,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_MAIN2, 0, 1, 0);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsAbleToRemoveAsCost()
    card::is_race(d, c, RACE_DINOSAUR) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.spfilter(c,e,tp,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsLevelBelow(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DINOSAUR) && card::is_level_below(d, c, lv as u32) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local cg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
        let cg = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_GRAVE, 0, NO_EX);
        let ncg = cg.len() as i64;
        // return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,0) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,#cg)
        return bfgcost(d, a, 0)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, ncg)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local cg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    let cg = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_GRAVE, 0, NO_EX);
    let ncg = cg.len() as i64;
    // local tg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,#cg)
    let tg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp, ncg)), tp, LOCATION_DECK, 0, NO_EX);
    // local lvt={}
    // local tc=tg:GetFirst()
    let _ = group::get_first(d, &tg);
    // for tc in aux.Next(tg) do
    //     local tlv=0
    //     tlv=tlv+tc:GetLevel()
    //     lvt[tlv]=tlv
    // end
    // The loop in Lua compacts the levels 1..12 into lvt[1..pc-1] in ascending order.
    let mut lvt: Vec<u64> = Vec::new();
    for tc in group::iter(d, &tg) {
        let tlv = card::get_level(d, tc) as u64;
        if (1..=12).contains(&tlv) && !lvt.contains(&tlv) {
            lvt.push(tlv);
        }
    }
    lvt.sort_unstable();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local lv=Duel.AnnounceNumber(tp,table.unpack(lvt))
    let lv = duel::announce_number(d, tp, &lvt).nil().0 as i64;
    // local rg1=Group.CreateGroup()
    let mut rg1 = CardSet::new();
    // if lv>1 then
    if lv > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local rg2=cg:Select(tp,lv-1,lv-1,c)
        let rg2 = group::select(d, &cg, tp, (lv - 1) as u16, (lv - 1) as u16, false, c.into()).nil();
        // rg1:Merge(rg2)
        group::add_card(&mut rg1, &rg2);
    }
    // rg1:AddCard(c)
    group::add_card(&mut rg1, &CardSet::from_card(c));
    // Duel.Remove(rg1,POS_FACEUP,REASON_COST)
    duel::remove(d, rg1, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(lv)
    effect::set_label(d, e, &[lv]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.sfilter(c,e,tp,lv)`
fn sfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:GetLevel()==lv and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DINOSAUR) && card::get_level(d, c) as i64 == lv && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.sfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,lv)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| sfilter(d, c, e, tp, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
            let h = effect::get_handler(d, e).nil();
            // local fid=e:GetHandler():GetFieldID()
            let fid = card::get_field_id(d, h) as i64;
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetLabel(fid)
            effect::set_label(d, e1, &[fid]);
            // e1:SetLabelObject(tc)
            effect::set_label_object(d, e1, Value::Card(tc));
            // e1:SetCondition(s.descon)
            effect::set_condition(d, e1, Some(condition(descon)));
            // e1:SetOperation(s.desop)
            effect::set_operation(d, e1, Some(operation(desop)));
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)==e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() == Some(effect::get_label(d, e)) {
        // return true
        true
    } else {
        // e:Reset()
        effect::reset(d, e);
        // return false
        false
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, tc, REASON_EFFECT, None, None);
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` called from a cost with the same arguments
fn bfgcost(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    aux::bfgcost()
        .call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(chk),
            ],
        )
        .truthy()
}
