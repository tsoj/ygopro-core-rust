//! Tri-Edge Master
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 52445243;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Apply effect(s) based on materials
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetLabel()>0 and e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(cond_e1)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Check materials used for Synchro Summon
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1a, EFFECT_MATERIAL_CHECK);
    // e1a:SetLabelObject(e1)
    effect::set_label_object(d, e1a, Value::Effect(e1));
    // e1a:SetValue(s.matcheck)
    effect::set_value_fn(d, e1a, effect_card_value(matcheck));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
}
/// `function(e) return e:GetLabel()>0 and e:GetHandler():IsSynchroSummoned() end`
fn cond_e1(d: &mut Duel, a: &EffArgs) -> bool {
    effect::get_label(d, a.e) > 0 && card::is_synchro_summoned(d, effect::get_handler(d, a.e).nil())
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local lb=e:GetLabel()
    let lb = effect::get_label(d, e);
    let c = effect::get_handler(d, e).nil();
    // local g=(lb&0x1>0) and Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler()) or nil
    let g = if lb & 0x1 > 0 {
        Some(duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into()))
    } else {
        None
    };
    // local dr=lb&0x2>0
    let dr = lb & 0x2 > 0;
    // if chk==0 then return (not g or #g>0) and (not dr or Duel.IsPlayerCanDraw(tp,1)) end
    if a.chk == Some(0) {
        return (g.as_ref().is_none_or(|g| !g.is_empty())) && (!dr || duel::is_player_can_draw(d, tp, 1));
    }
    // if g then Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0) end
    if let Some(g) = &g {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), 1, 0, 0);
    }
    // if dr then
    if dr {
        // if lb==0x2 then
        //     Duel.SetTargetPlayer(tp)
        //     Duel.SetTargetParam(1)
        // end
        if lb == 0x2 {
            duel::set_target_player(d, tp);
            duel::set_target_param(d, 1);
        }
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    // local index=(lb==0x1 and 1) or (lb==0x2 and 2) or (lb==0x4 and 3) or 4
    let index = if lb == 0x1 {
        1
    } else if lb == 0x2 {
        2
    } else if lb == 0x4 {
        3
    } else {
        4
    };
    // Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,index))
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, aux::stringid(ID, index));
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local lb=e:GetLabel()
    let lb = effect::get_label(d, e);
    // local breakeff=false
    let mut breakeff = false;
    // Destroy 1 other card on the field
    // if lb&0x1>0 then
    if lb & 0x1 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
        // if #g>0 then
        if !g.is_empty() {
            // breakeff=true
            breakeff = true;
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
    // Draw 1 card
    // if lb&0x2>0 then
    if lb & 0x2 > 0 {
        // local p,d=tp,1
        let mut p = tp;
        let mut cnt: u32 = 1;
        // if lb==0x2 then p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM) end
        if lb == 0x2 {
            let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
            p = info[0].int() as u8;
            cnt = info[1].int() as u32;
        }
        // if Duel.IsPlayerCanDraw(p) then
        if duel::is_player_can_draw(d, p, 0) {
            // if breakeff then Duel.BreakEffect() end
            if breakeff {
                duel::break_effect(d);
            }
            // breakeff=true
            breakeff = true;
            // Duel.Draw(p,d,REASON_EFFECT)
            duel::draw(d, p, cnt, REASON_EFFECT);
        }
    }
    // Treat this card as a Tuner
    // if lb&0x4>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if lb & 0x4 > 0 && card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) {
        // if breakeff then Duel.BreakEffect() end
        if breakeff {
            duel::break_effect(d);
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local e1=e:GetLabelObject()
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // if #g>2 then
    if g.len() > 2 {
        // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
        effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
        // e1:SetLabel(0x1|0x2|0x4)
        effect::set_label(d, e1, &[0x1 | 0x2 | 0x4]);
    } else if g.len() == 2 {
        // local a=g:GetFirst():GetSynchroLevel(c)
        // local b=g:GetNext():GetSynchroLevel(c)
        let cards: Vec<CardId> = group::iter(d, &g);
        let a = card::get_synchro_level(d, cards[0], c) as i64;
        let b = card::get_synchro_level(d, cards[1], c) as i64;
        // if a+b~=6 then return end
        if a + b != 6 {
            return 0;
        }
        // local lv=math.min(a,b)
        let lv = a.min(b);
        // if lv==1 then e1:SetCategory(CATEGORY_DESTROY)
        // elseif lv==2 then e1:SetCategory(CATEGORY_DRAW) end
        if lv == 1 {
            effect::set_category(d, e1, CATEGORY_DESTROY);
        } else if lv == 2 {
            effect::set_category(d, e1, CATEGORY_DRAW);
        }
        // e1:SetLabel(1<<(lv-1))
        // (Lua shifts: a negative count shifts right)
        let sh = lv - 1;
        let label = if sh >= 0 { 1i64 << sh } else { 1i64 >> (-sh) };
        effect::set_label(d, e1, &[label]);
    }
    0
}
