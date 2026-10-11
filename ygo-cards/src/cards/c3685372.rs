//! CXyz Gimmick Puppet Fanatix Machinix
use super::prelude::*;

pub const ID: u32 = 3685372;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PUPPET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,9,3)
    xyz::add_procedure(d, c, None, Some(9), 3, None, None, None, None, false, None);
    // Add 1 "Puppet" Trap card from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 monster from either GY to your opponent's field in Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 monster summoned to the opponent's field and inflict damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function() return not Duel.IsPhase(PHASE_DAMAGE) end)
    effect::set_condition(d, e3, Some(condition(not_damage_phase_cond)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Group(CardSet::new()));
    // Register monsters summoned to the opponent's field
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3a, EVENT_SPSUMMON_SUCCESS);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetLabelObject(e3)
    effect::set_label_object(d, e3a, Value::Effect(e3));
    // e3a:SetOperation(s.regop)
    effect::set_operation(d, e3a, Some(operation(regop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) for e3: not Duel.IsPhase(PHASE_DAMAGE)
fn not_damage_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    !duel::is_phase(d, PHASE_DAMAGE)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PUPPET) and c:IsTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PUPPET as u16]) && card::is_trap(d, c) && card::is_able_to_hand(d, c, None)
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
    if g.len() > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,1-tp)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp) end
    if a.chk == Some(0) {
        let (e, tp) = (a.e, a.tp);
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)<=0 then return end
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SpecialSummon(g,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
// function s.desfilter1(c,e,tp)
fn desfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsCanBeEffectTarget(e)
    card::is_controler(d, c, 1 - tp) && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tg=eg:Filter(s.desfilter1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, x| desfilter1(d, x, e, tp), NO_EX);
    // if #tg>0 then
    if tg.len() > 0 {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, x| card::get_flag_effect(d, x, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
// function s.desfilter2(c,tp,e)
fn desfilter2(d: &mut Duel, c: CardId, tp: u8, e: Option<EffectId>) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(1-tp) and (not e or c:IsCanBeEffectTarget(e))
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, 1 - tp) && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=e:GetLabelObject():Filter(s.desfilter2,nil,tp,e)
    let stored = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &stored, &|d, x| desfilter2(d, x, tp, Some(e)), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.desfilter2(chkc,tp,nil) end
    if let Some(chkc) = a.chkc() {
        return g.contains(chkc) && desfilter2(d, chkc, tp, None);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return g.len() > 0;
    }
    // local tc=nil
    let tc = if g.len() == 1 {
        // tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
        tc
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
        tc
    };
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, tp, 0);
    // if tc:IsFaceup() and tc:GetBaseAttack()>0 then
    if card::is_faceup(d, tc) && card::get_base_attack(d, tc) > 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,tc:GetBaseAttack()//2)
        let half = card::get_base_attack(d, tc) / 2;
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - tp, half);
    } else {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - tp, 0);
    }
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and tc:GetBaseAttack()>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0
        && card::get_base_attack(d, tc) > 0
    {
        // Duel.Damage(1-tp,tc:GetBaseAttack()//2,REASON_EFFECT)
        let half = card::get_base_attack(d, tc) / 2;
        duel::damage(d, 1 - a.tp, half as i64, REASON_EFFECT, false, None);
    }
}
