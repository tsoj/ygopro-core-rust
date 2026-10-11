//! Overload Future Fusion
use super::prelude::*;

pub const ID: u32 = 33099732;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.exfilter(c,tp)`
fn exfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c.material and c:IsType(TYPE_FUSION) and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,c)
    let mat = crate::class::material(d, c);
    !mat.is_empty() && card::is_type(d, c, TYPE_FUSION) && duel::is_existing_matching_card(d, Some(&|d, cc| tgfilter(d, cc, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.tgfilter(c,fc)`
fn tgfilter(d: &mut Duel, c: CardId, fc: CardId) -> bool {
    // return c:IsCode(table.unpack(fc.material)) and c:IsAbleToGrave()
    let mat = crate::class::material(d, fc);
    card::is_code(d, c, &mat) && card::is_able_to_grave(d, c, None)
}
/// `s.ffilter(c)`
fn ffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_MACHINE)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_MACHINE)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
        return (Some(g), None);
    }
    // return nil
    (None, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `fusparams` of the script: `{handler=c,fusfilter=s.ffilter,matfilter=aux.FALSE,extrafil=s.fextra,extraop=Fusion.BanishMaterial,extratg=s.extratg}`
fn fusparams() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| ffilter(d, c))),
        matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
        extrafil: Some(std::sync::Arc::new(fextra)),
        extraop: Some(fusion::banish_material()),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of the script's target or operation
fn fusion_args(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local fusparams={...}
    // local b1=not Duel.HasFlagEffect(tp,id) and Duel.IsExistingMatchingCard(s.exfilter,tp,LOCATION_EXTRA,0,1,nil,tp)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_existing_matching_card(d, Some(&|d, c| exfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Fusion.SummonEffTG(fusparams)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None) && {
        let mut args = fusion_args(a);
        args.push(Value::Int(0));
        fusion::summon_eff_tg(d, fusparams()).call(d, &args).truthy()
    };
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    } else if op == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Fusion.SummonEffTG(fusparams)(e,tp,eg,ep,ev,re,r,rp,1)
        let mut args = fusion_args(a);
        args.push(Value::Int(1));
        fusion::summon_eff_tg(d, fusparams()).call(d, &args);
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON+CATEGORY_REMOVE)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON + CATEGORY_REMOVE);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Send 1 Fusion Material from your Deck to the GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local cc=Duel.SelectMatchingCard(tp,s.exfilter,tp,LOCATION_EXTRA,0,1,1,nil,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| exfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if not cc then return end
        let cc = match group::get_first(d, &sel) {
            Some(cc) => cc,
            None => return,
        };
        // Duel.ConfirmCards(1-tp,cc)
        duel::confirm_cards(d, 1 - tp, cc);
        // Duel.ShuffleExtra(tp)
        duel::shuffle_extra(d, tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil,cc):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, cc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if tc and Duel.SendtoGrave(tc,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_GRAVE) then
        if let Some(tc) = tc {
            if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_GRAVE) {
                let code = card::get_code(d, tc).first().copied().unwrap_or(0);
                // Cannot Special Summon monsters with the same name
                // local e1=Effect.CreateEffect(c)
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(aux.Stringid(id,3))
                effect::set_description(d, e1, aux::stringid(ID, 3));
                // e1:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD);
                // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
                effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
                // e1:SetTargetRange(1,0)
                effect::set_target_range(d, e1, 1, 0);
                // e1:SetTarget(function(e,c) return c:IsCode(e:GetLabel()) end)
                effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| card::is_code(d, c, &[effect::get_label(d, e) as u32]))));
                // e1:SetLabel(tc:GetCode())
                effect::set_label(d, e1, &[code as i64]);
                // e1:SetReset(RESET_PHASE|PHASE_END)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, tp);
                // Cannot activate monster effects of cards with the same name
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, h);
                // e2:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e2, EFFECT_TYPE_FIELD);
                // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
                effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
                // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
                effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
                // e2:SetTargetRange(1,0)
                effect::set_target_range(d, e2, 1, 0);
                // e2:SetValue(function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsCode(e:GetLabel()) end)
                effect::set_value_fn(d, e2, lua_fn(cannot_activate_value));
                // e2:SetLabel(tc:GetCode())
                effect::set_label(d, e2, &[code as i64]);
                // e2:SetReset(RESET_PHASE|PHASE_END)
                effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
                // Duel.RegisterEffect(e2,tp)
                duel::register_effect(d, e2, tp);
            }
        }
    } else if op == 2 {
        // Fusion Summon 1 DARK Machine Fusion Monster
        // Fusion.SummonEffOP(fusparams)(e,tp,eg,ep,ev,re,r,rp)
        let args = fusion_args(a);
        fusion::summon_eff_op(d, fusparams()).call(d, &args);
    }
}
/// `function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsCode(e:GetLabel()) end` (the value of e2)
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // re:IsMonsterEffect()
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    // re:GetHandler():IsCode(e:GetLabel())
    let h = effect::get_handler(d, re).nil();
    card::is_code(d, h, &[effect::get_label(d, e) as u32])
}
